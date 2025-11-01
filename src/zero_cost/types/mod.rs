use simd_json::BorrowedValue;

pub mod abstraction;
pub mod helpers;
pub mod macros;
pub mod methods;
pub mod wrappers;

pub static NULL: BorrowedValue = BorrowedValue::Static(simd_json::StaticNode::Null);
