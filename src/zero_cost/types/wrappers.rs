/*

Std wrappers

*/

use std::collections::HashMap;

use simd_json::{
    BorrowedValue,
    base::{ValueAsArray, ValueAsObject, ValueAsScalar},
    derived::TypedScalarValue,
};

use crate::{zc_wrap, zc_wrapper};

zc_wrapper! {
    ZcBool,
    ZcI32,
    ZcIsize,
    ZcStr,
    ZcOption<T>,
    ZcVec<T>,
    ZcHashMap<T>
}

/*

Std wrap

*/
pub trait IntoZcType<T> {
    fn zc_type(&self) -> T;
}

pub trait FromBorrowedValue<'a>: Sized {
    fn from_borrowed_value(value: &'a BorrowedValue<'a>) -> Self;
}

zc_wrap! {
    // Std
    ZcBool,
    ZcI32,
    ZcIsize,
    ZcStr,
    ZcOption<T>,
    ZcVec<T>,
    ZcHashMap<T>
}

/*

Std unwrap

*/
pub trait ZcType<T> {
    fn std_type(&self) -> T;
}

impl<'a> ZcType<&'a str> for ZcStr<'a> {
    fn std_type(&self) -> &'a str {
        self.inner.as_str().expect("Not a valid string")
    }
}

impl<'a> ZcType<isize> for ZcIsize<'a> {
    fn std_type(&self) -> isize {
        self.inner.as_i64().expect("not a valid isize") as isize
    }
}

impl<'a, T> ZcType<HashMap<String, T>> for ZcHashMap<'a, T>
where
    T: FromBorrowedValue<'a>,
{
    fn std_type(&self) -> HashMap<String, T> {
        self.inner
            .as_object()
            .expect("Not a valid map")
            .into_iter()
            .map(|(k, v)| (k.to_string(), T::from_borrowed_value(v)))
            .collect()
    }
}

impl<'a, T> ZcType<Vec<T>> for ZcVec<'a, T>
where
    T: FromBorrowedValue<'a>,
{
    fn std_type(&self) -> Vec<T> {
        self.inner
            .as_array()
            .expect("Not a valid array")
            .into_iter()
            .map(|v| T::from_borrowed_value(v))
            .collect()
    }
}

pub struct ZcIter<'a, T> {
    inner: ZcVec<'a, T>,
    position: usize,
}

impl<T> ZcVec<'_, T> {
    pub fn len(&self) -> Option<usize> {
        match &self.inner {
            BorrowedValue::Array(values) => Some(values.len()),
            _ => None,
        }
    }
}

impl<'a, T> IntoIterator for ZcVec<'a, T>
where
    T: FromBorrowedValue<'a>,
{
    type Item = T;

    type IntoIter = ZcIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            inner: self,
            position: 0,
        }
    }
}

impl<'a, T> Iterator for ZcIter<'a, T>
where
    T: FromBorrowedValue<'a>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.inner.len()? {
            let cache = self.position;
            self.position += 1;
            match &self.inner.inner {
                BorrowedValue::Array(values) => {
                    values.get(cache).map(|v| T::from_borrowed_value(v))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

impl<'a, T> ZcType<Option<T>> for ZcOption<'a, T>
where
    T: FromBorrowedValue<'a>,
{
    fn std_type(&self) -> Option<T> {
        if self.inner.is_null() {
            None
        } else {
            Some(T::from_borrowed_value(&self.inner))
        }
    }
}

impl<'a> ZcType<bool> for ZcBool<'a> {
    fn std_type(&self) -> bool {
        self.inner.as_bool().expect("Not a valid bool")
    }
}
