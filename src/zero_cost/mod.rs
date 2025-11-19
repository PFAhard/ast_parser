pub mod types;
pub mod convers;

use core::cell::Cell;
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

use crate::zero_cost::types::{abstraction::ZcSourceUnit, wrappers::FromBorrowedValue};

pub trait BorrowedValueVisitor<'a> {
    fn filter_by_id(&'a self, id: isize) -> Option<&'a BorrowedValue<'a>>;

    fn filter_by_ref_id(&'a self, ref_id: isize) -> Vec<&'a BorrowedValue<'a>>;

    fn filter_by_node_type(&'a self, node_type: &str) -> Vec<&'a BorrowedValue<'a>>;

    fn iter_by_node_type(&'a self, node_type: &str) -> impl Iterator<Item = &'a BorrowedValue<'a>> /*  + Debug */;

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

    fn children(&'a self) -> Vec<&'a BorrowedValue<'a>>;

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

    /// Still allocates vector, but shorter, normal one allocate pointer for each node, this one allocate only n (depth) elements (+ x2 amortized etc. as always in vector)
    fn iter_by_node_type(&'a self, node_type: &str) -> impl Iterator<Item = &'a BorrowedValue<'a>> /* + Debug */
    {
        struct InnerIter<'iter> {
            node: &'iter BorrowedValue<'iter>,
            relative_pos: Cell<usize>,
        }

        // TODO: Hide behind feature
        // impl core::fmt::Debug for InnerIter<'_> {
        //     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        //         let statical = match &self.node {
        //             BorrowedValue::Static(static_node) => Some(static_node.to_string()),
        //             BorrowedValue::String(cow) => Some(cow.to_string()),
        //             _ => None,
        //         }
        //         .unwrap_or_default();

        //         let node_type = self
        //             .node
        //             .get_key("nodeType")
        //             .map(|v| v.as_str())
        //             .flatten()
        //             .unwrap_or("not defined");
        //         let id = self
        //             .node
        //             .get_key("id")
        //             .map(|v| v.as_i64())
        //             .flatten()
        //             .unwrap_or_default();
        //         let name = self
        //             .node
        //             .get_key("name")
        //             .map(|v| v.as_str())
        //             .flatten()
        //             .unwrap_or("not defined");

        //         match self {
        //             InnerIter { node, relative_pos } => f
        //                 .debug_struct("InnerIter")
        //                 .field(
        //                     "node",
        //                     &format!(
        //                         "Node: id: `{}`; name: `{}`; node type: `{}`. Static: `{}`;",
        //                         id, name, node_type, statical
        //                     ),
        //                 )
        //                 .field("relative_pos", &relative_pos)
        //                 .finish(),
        //         }
        //     }
        // }

        // #[derive(Debug)]
        struct NodeTypeIter<'a, 'iter> {
            stack: Vec<InnerIter<'iter>>,
            node_type: &'a str,
        }

        impl<'a, 'iter> Iterator for NodeTypeIter<'a, 'iter> {
            type Item = &'iter BorrowedValue<'iter>;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    dbg!(self.stack.capacity());
                    // dbg!(&self);
                    if self.stack.is_empty() {
                        return None;
                    }

                    let last = self.stack.last()?;
                    let rel_pos = last.relative_pos.get();

                    let mut has_return = None;

                    match &last.node {
                        BorrowedValue::Array(values) => {
                            let length = values.len();
                            if rel_pos >= length {
                                self.stack.pop();
                                continue;
                            } else {
                                last.relative_pos.set(rel_pos + 1);
                                self.stack.push(InnerIter {
                                    node: &values[rel_pos],
                                    relative_pos: Cell::new(0),
                                });
                                continue;
                            }
                        }
                        BorrowedValue::Object(map) => {
                            if let Some(object_type) = map.get("nodeType")
                                && object_type.is_string(self.node_type)
                            {
                                has_return = Some(last.node);
                            }

                            let mut values = map.values();
                            let nth = values.nth(rel_pos);
                            if let Some(nth) = nth {
                                last.relative_pos.set(rel_pos + 1);
                                if has_return.is_some() {
                                    self.stack.pop();
                                }
                                self.stack.push(InnerIter {
                                    node: nth,
                                    relative_pos: Cell::new(0),
                                });
                                if has_return.is_some() {
                                    return has_return;
                                } else {
                                    continue;
                                }
                            } else {
                                self.stack.pop();
                                if has_return.is_some() {
                                    return has_return;
                                } else {
                                    continue;
                                }
                            }
                        }
                        _ => {
                            self.stack.pop();
                            continue;
                        }
                    }
                }
            }
        }

        NodeTypeIter {
            stack: vec![InnerIter {
                node: &self,
                relative_pos: Cell::new(0),
            }],
            node_type,
        }
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

    fn filter_by_ref_id(&'a self, ref_id: isize) -> Vec<&'a BorrowedValue<'a>> {
        match self {
            BorrowedValue::Array(values) => values
                .iter()
                .flat_map(|inner| inner.filter_by_ref_id(ref_id))
                .collect(),
            BorrowedValue::Object(sized_hash_map) => {
                let map = sized_hash_map
                    .values()
                    .flat_map(|inner| inner.filter_by_ref_id(ref_id));

                if let Some(object_type) = sized_hash_map.get("referencedDeclaration")
                    && object_type.is_number(ref_id)
                {
                    std::iter::once(self).chain(map).collect()
                } else {
                    map.collect()
                }
            }
            _ => vec![],
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

    fn children(&'a self) -> Vec<&'a BorrowedValue<'a>> {
        let mut acc = vec![];
        match self {
            BorrowedValue::Array(values) => {
                acc.extend(values.iter().flat_map(BorrowedValueVisitor::children))
            }
            BorrowedValue::Object(sized_hash_map) => {
                acc.push(self);
                acc.extend(
                    sized_hash_map
                        .values()
                        .flat_map(BorrowedValueVisitor::children),
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

pub struct SourceUnitBuilder<'a> {
    map: MmapMut,
    root: Option<BorrowedValue<'a>>,
}

impl SourceUnitBuilder<'static> {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let file = File::options().read(true).open(path).unwrap();
        let mmap = unsafe { MmapOptions::new().map_copy(&file).unwrap() };

        let mut this = Self {
            map: mmap,
            root: None,
        };

        this.get_root();

        this
    }

    pub fn get_root(&mut self) {
        let slice: &mut [u8] = &mut self.map;

        // SAFETY: BorrowedValue only borrows from map, which lives as long as self.
        let root: BorrowedValue<'static> = match to_borrowed_value(slice) {
            Ok(root) => unsafe {
                std::mem::transmute::<BorrowedValue<'_>, BorrowedValue<'static>>(root)
            },
            Err(err) => {
                unimplemented!("Unhandled error occurs: {}", err)
            }
        };

        self.root = Some(root);
    }

    pub fn source_unit<const N: usize>(&'_ self, key: Option<[&str; N]>) -> ZcSourceUnit<'_> {
        ZcSourceUnit {
            inner: match key {
                Some(key) => self
                    .root
                    .as_ref()
                    .expect("get_root() must be called first")
                    .get_chain(key)
                    .unwrap(),
                None => self.root.as_ref().unwrap(),
            },
        }
    }

    /// Technically safe, since file always persist in the system if not modified, which is UBs
    pub fn source_unit_const<const N: usize>(
        &self,
        key: Option<[&str; N]>,
    ) -> ZcSourceUnit<'static> {
        let su = self.source_unit(key);
        unsafe { std::mem::transmute(su) }
    }

    /// # Safety:
    /// Only use if you know what are you doing
    pub unsafe fn arbitrary_access<const N: usize>(
        &'_ self,
        key: Option<[&str; N]>,
    ) -> &BorrowedValue<'_> {
        match key {
            Some(key) => self
                .root
                .as_ref()
                .expect("get_root() must be called first")
                .get_chain(key)
                .unwrap(),
            None => self.root.as_ref().expect("get_root() must be called first"),
        }
    }
}

pub struct NodeBuilder {
    map: MmapMut,
    node: Option<BorrowedValue<'static>>,
}

impl NodeBuilder {
    pub fn new<P>(path: P, start: u32, length: u32) -> Self
    where
        P: AsRef<Path>,
    {
        let file = File::options().read(true).open(path).unwrap();
        let mmap = unsafe {
            MmapOptions::new()
                .offset(start as u64)
                .len(length as usize)
                .map_copy(&file)
                .unwrap()
        };

        let mut this = Self {
            map: mmap,
            node: None,
        };

        this.get_node();

        this
    }

    pub fn get_node(&mut self) {
        let slice: &mut [u8] = &mut self.map;

        // SAFETY: BorrowedValue only borrows from map, which lives as long as self.
        let root: BorrowedValue<'static> = unsafe {
            std::mem::transmute::<BorrowedValue<'_>, BorrowedValue<'static>>(
                to_borrowed_value(slice).expect("invalid JSON"),
            )
        };

        self.node = Some(root);
    }

    pub fn strong_node<T>(&self) -> T
    where
        T: FromBorrowedValue<'static>,
    {
        let node = self.node.as_ref().expect("must call `get_node` first");
        let node: &'static BorrowedValue<'static> = unsafe { std::mem::transmute(node) };
        T::from_borrowed_value(node)
    }
}
