pub mod types;

use std::{fs::File, path::Path};

use memmap2::{MmapMut, MmapOptions};
use simd_json::{
    BorrowedValue, StaticNode,
    derived::{TypedArrayValue, TypedObjectValue, ValueObjectAccess},
    to_borrowed_value,
};

pub use simd_json::base::{
    ValueAsArray, ValueAsMutArray, ValueAsMutObject, ValueAsObject, ValueAsScalar,
};

use crate::zero_cost::types::abstraction::ZcSourceUnit;

pub trait BorrowedValueVisitor<'a> {
    fn filter_by_id(&'a self, id: isize) -> Option<&'a BorrowedValue<'a>>;

    fn filter_by_ref_id(&'a self, ref_id: isize) -> Option<&'a BorrowedValue<'a>>;

    fn filter_by_node_type(&'a self, node_type: &str) -> Vec<&'a BorrowedValue<'a>>;

    fn step_back_to_node_type(
        &'a self,
        anchor: &'a BorrowedValue<'a>,
        node_type: &str,
    ) -> Option<&'a BorrowedValue<'a>>;

    fn step_back(&'a self, anchor: &'a BorrowedValue<'a>) -> Option<&'a BorrowedValue<'a>>;

    fn direct_children(&'a self) -> Vec<&'a BorrowedValue<'a>>;

    fn is_node_id(&self, id: isize) -> Option<bool>;

    fn is_any_node_id(&'a self, id: isize) -> Option<&'a BorrowedValue<'a>>;

    fn children_ids(&self) -> Vec<isize>;

    fn is_string(&self, value: &str) -> bool;

    fn is_number(&self, value: isize) -> bool;

    fn check_key<T>(&self, key: &str, check: T) -> bool
    where
        T: FnOnce(&BorrowedValue<'_>) -> bool;

    fn check_chain<const N: usize, T>(&self, chain: [&str; N], check: T) -> bool
    where
        T: FnOnce(&BorrowedValue<'_>) -> bool;

    fn get_key(&'a self, key: &str) -> Option<&'a BorrowedValue<'a>>;

    fn get_chain<const N: usize>(&'a self, chain: [&str; N]) -> Option<&'a BorrowedValue<'a>>;
}

impl<'a> BorrowedValueVisitor<'a> for BorrowedValue<'a> {
    fn filter_by_node_type(&'a self, node_type: &str) -> Vec<&'a BorrowedValue<'a>> {
        let mut acc = vec![];
        match self {
            BorrowedValue::Array(values) => {
                values
                    .iter()
                    .for_each(|inner| acc.extend(inner.filter_by_node_type(node_type)));
            }
            BorrowedValue::Object(sized_hash_map) => {
                if let Some(object_type) = sized_hash_map.get("nodeType")
                    && object_type.is_string(node_type)
                {
                    acc.push(self);
                }

                sized_hash_map
                    .values()
                    .for_each(|inner| acc.extend(inner.filter_by_node_type(node_type)));
            }
            _ => {}
        };

        acc
    }

    fn step_back_to_node_type(
        &'a self,
        anchor: &'a BorrowedValue<'a>,
        node_type: &str,
    ) -> Option<&'a BorrowedValue<'a>> {
        self.step_back(anchor)
            .map(|parent| {
                let is_target_node = parent
                    .get("nodeType")
                    .map(|nt| nt.as_str().map(|nt| nt == node_type))
                    .flatten()
                    .is_some_and(|x| x);
                if is_target_node {
                    Some(parent)
                } else {
                    parent.step_back_to_node_type(anchor, node_type)
                }
            })
            .flatten()
    }

    /// Ok, attempt №2
    /// As input we have self => Target node and anchor => root node
    /// Out task is to find a way from anchor to target.
    /// Returning the last node before target, i.e. parent node
    ///
    /// I assume, all works is done with objects. Array must be in inplaced, to avoid anonymous nodes
    ///
    fn step_back(&'a self, anchor: &'a BorrowedValue<'a>) -> Option<&'a BorrowedValue<'a>> {
        if std::ptr::eq(anchor, self) {
            // WE are the anchor
            return None;
        }

        if !anchor.is_object() {
            // @note since when not => we couldn't care less
            return None;
        }

        // So, if object has target as a DIRECT child, it is the parent
        // IF object has an array that has a DIRECT child, object is the parent
        // IF object has an array1, that has an array2 ... that has an arrayN, object is the parent
        // Otherwise we must traverse INLINED children ({DIRECT CHILD SET} + {DIRECT ARRAY'S VALUES}) to find, which of them is the parent
        // IF INLINED child is a string, we do'not care, if it is static object (number, bool etc.) we do not care.
        // Invriant => is never array, since all arrays values are INLINED as direct object children
        let object = anchor.as_object().expect("Checked is_object");

        for v in object.values() {
            if std::ptr::eq(v, self) {
                // @note The most direct child
                return Some(anchor);
            }
        }

        // If any array (possibly nested arrays) contains the target (as element) => anchor is the parent.
        fn array_contains_target_recursive<'a>(
            arr_val: &'a BorrowedValue<'a>,
            target: &'a BorrowedValue<'a>,
        ) -> bool {
            if !arr_val.is_array() {
                return false;
            }
            let arr = arr_val.as_array().expect("is_array checked");
            for elem in arr.iter() {
                if std::ptr::eq(elem, target) {
                    return true;
                }
                if elem.is_array() && array_contains_target_recursive(elem, target) {
                    return true;
                }
            }
            false
        }

        for v in object.values() {
            if v.is_array() && array_contains_target_recursive(v, self) {
                // Child of the (child array)
                return Some(anchor);
            }
        }

        // Otherwise, traverse "inlined children" (direct object children and array values)
        for v in object.values() {
            // If `v` is object: recursively try to find parent inside it.
            if v.is_object() {
                if let Some(found_parent) = self.step_back(v) {
                    return Some(found_parent);
                }
            }

            // If `v` is array: attempt to search each element (which are considered inlined children).
            if v.is_array() {
                let arr = v.as_array().expect("is_array checked");
                for elem in arr.iter() {
                    let inlined = elem.direct_children();
                    for elem in inlined {
                        if let Some(found_parent) = self.step_back(elem) {
                            return Some(found_parent);
                        }
                    }
                }
            }
        }

        None
    }

    /// Children objects
    fn direct_children(&'a self) -> Vec<&'a BorrowedValue<'a>> {
        match self {
            Self::Static(static_node) => vec![],
            Self::String(cow) => vec![],
            Self::Array(values) => values.iter().flat_map(|v| v.direct_children()).collect(),
            Self::Object(sized_hash_map) => {
                vec![self]
            }
        }
    }

    fn is_node_id(&self, id: isize) -> Option<bool> {
        unimplemented!("Remainings of First attempt")
    }

    fn is_any_node_id(&'a self, id: isize) -> Option<&'a BorrowedValue<'a>> {
        unimplemented!("Remainings of First attempt")
    }

    fn is_string(&self, value: &str) -> bool {
        match self {
            BorrowedValue::String(cow) => cow == value,
            _ => false,
        }
    }

    fn filter_by_id(&'a self, id: isize) -> Option<&'a BorrowedValue<'a>> {
        match self {
            BorrowedValue::Array(values) => values.iter().find_map(|inner| inner.filter_by_id(id)),
            BorrowedValue::Object(sized_hash_map) => {
                if let Some(object_type) = sized_hash_map.get("id")
                    && object_type.is_number(id)
                {
                    return Some(self);
                }

                sized_hash_map
                    .values()
                    .find_map(|inner| inner.filter_by_id(id))
            }
            _ => None,
        }
    }

    fn filter_by_ref_id(&'a self, ref_id: isize) -> Option<&'a BorrowedValue<'a>> {
        match self {
            BorrowedValue::Array(values) => values
                .iter()
                .find_map(|inner| inner.filter_by_ref_id(ref_id)),
            BorrowedValue::Object(sized_hash_map) => {
                if let Some(object_type) = sized_hash_map.get("referencedDeclaration")
                    && object_type.is_number(ref_id)
                {
                    return Some(self);
                }

                sized_hash_map
                    .values()
                    .find_map(|inner| inner.filter_by_ref_id(ref_id))
            }
            _ => None,
        }
    }

    fn is_number(&self, value: isize) -> bool {
        match self {
            BorrowedValue::Static(static_node) => match static_node {
                StaticNode::I64(i) => *i as isize == value,
                StaticNode::U64(u) => *u as isize == value,
                StaticNode::F64(f) => *f as isize == value,
                _ => false,
            },
            _ => false,
        }
    }

    fn get_key(&'a self, key: &str) -> Option<&'a BorrowedValue<'a>> {
        self.as_object().map(|x| x.get(key)).flatten()
    }

    fn get_chain<const N: usize>(&'a self, chain: [&str; N]) -> Option<&'a BorrowedValue<'a>> {
        let mut s = self;
        for key in chain {
            s = s.get_key(key)?;
        }

        Some(s)
    }

    fn children_ids(&self) -> Vec<isize> {
        let mut acc = vec![];
        match self {
            BorrowedValue::Array(values) => {
                acc.extend(values.iter().flat_map(BorrowedValueVisitor::children_ids))
            }
            BorrowedValue::Object(sized_hash_map) => {
                if let Some(id) = sized_hash_map.get("id") {
                    acc.push(id.as_i64().unwrap() as isize);
                }
                acc.extend(
                    sized_hash_map
                        .values()
                        .flat_map(BorrowedValueVisitor::children_ids),
                )
            }
            _ => {}
        }

        acc
    }

    fn check_key<T>(&self, key: &str, check: T) -> bool
    where
        T: FnOnce(&BorrowedValue<'_>) -> bool,
    {
        self.get_key(key).is_some_and(check)
    }

    fn check_chain<const N: usize, T>(&self, chain: [&str; N], check: T) -> bool
    where
        T: FnOnce(&BorrowedValue<'_>) -> bool,
    {
        self.get_chain(chain).is_some_and(check)
    }
}

pub struct SourceUnitBuilder {
    map: MmapMut,
    root: Option<BorrowedValue<'static>>,
}

impl SourceUnitBuilder {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let file = File::options().read(true).open(path).unwrap();
        let mmap = unsafe { MmapOptions::new().map_copy(&file).unwrap() };

        Self {
            map: mmap,
            root: None,
        }
    }

    pub fn get_root(&mut self) {
        let slice: &mut [u8] = &mut self.map;

        // SAFETY: BorrowedValue only borrows from map, which lives as long as self.
        let root: BorrowedValue<'static> = unsafe {
            std::mem::transmute::<BorrowedValue<'_>, BorrowedValue<'static>>(
                to_borrowed_value(slice).expect("invalid JSON"),
            )
        };

        self.root = Some(root);
    }

    pub fn source_unit(&'_ self) -> ZcSourceUnit<'_> {
        ZcSourceUnit {
            inner: self
                .root
                .as_ref()
                .expect("get_root() must be called first")
                .get_key("ast")
                .unwrap(),
        }
    }

    /// Technically safe, since file always persist in the system if not modified, which is UBs
    pub fn source_unit_const(&self) -> ZcSourceUnit<'static> {
        let su = self.source_unit();
        unsafe { std::mem::transmute(su) }
    }
}
