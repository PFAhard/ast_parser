use std::{fs::File, path::Path};

use memmap2::{MmapMut, MmapOptions};
use simd_json::{
    BorrowedValue, StaticNode,
    base::{ValueAsObject, ValueAsScalar},
    to_borrowed_value,
};

pub trait BorrowedValueVisitor<'a> {
    fn filter_by_id(&'a self, id: isize) -> Option<&'a BorrowedValue<'a>>;

    fn filter_by_ref_id(&'a self, ref_id: isize) -> Option<&'a BorrowedValue<'a>>;

    fn filter_by_node_type(&self, node_type: &str) -> Vec<BorrowedValue<'a>>;

    fn children_ids(&self) -> Vec<isize>;

    fn is_string(&self, value: &str) -> bool;

    fn is_number(&self, value: isize) -> bool;

    fn get_key(&'a self, key: &str) -> Option<&'a BorrowedValue<'a>>;
}

impl<'a> BorrowedValueVisitor<'a> for BorrowedValue<'a> {
    fn filter_by_node_type(&self, node_type: &str) -> Vec<BorrowedValue<'a>> {
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
                    acc.push(self.clone());
                }

                sized_hash_map
                    .values()
                    .for_each(|inner| acc.extend(inner.filter_by_node_type(node_type)));
            }
            _ => {}
        };

        acc
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

    pub fn source_unit(&self) -> &BorrowedValue<'_> {
        self.root
            .as_ref()
            .expect("get_root() must be called first")
            .get_key("ast")
            .unwrap()
    }
}
