#[macro_export]
macro_rules! zc_abstract {
    (
        @struct $v:vis $name:ident {
            $(
                $member:ident $(=> [$key:literal])?: $member_type:ty,
            )*
        }
    ) => {
        paste::paste! {
            $v struct [< Zc $name >]<'a> {
                $v inner: &'a simd_json::BorrowedValue<'a>,
            }

            impl<'a> [< Zc $name >]<'a> {
                $(
                    $v fn $member(&'_ self) -> $member_type {
                        #[allow(unused_mut, unused_assignments)]
                        let mut key_handle = stringify!($member);
                        $(
                            key_handle = $key;
                        )?

                        $member_type::from_borrowed_value(self.inner.get_key(
                            key_handle
                        ).expect(stringify!([< Zc $name _ $member _ Broken_AST >])))
                    }
                )*

                pub fn filter_by_id_strong<T: FromBorrowedValue<'a>>(&'a self, id: isize) -> Option<T> {
                    self.inner.filter_by_id(id).map(T::from_borrowed_value)
                }

                pub fn filter_by_ref_id_strong<T: FromBorrowedValue<'a>>(&'a self, ref_id: isize) -> Option<T> {
                    self.inner.filter_by_ref_id(ref_id).map(T::from_borrowed_value)
                }

                pub fn filter_by_node_type_strong<T: FromBorrowedValue<'a>>(&'a self, node_type: &str) -> Vec<T> {
                    self.inner.filter_by_node_type(node_type).into_iter().map(T::from_borrowed_value).collect()
                }
            }

            impl<'a> BorrowedValueVisitor<'a> for [< Zc $name >]<'a> {
                fn filter_by_id(&'a self, id: isize) -> Option<&'a simd_json::BorrowedValue<'a>> {
                    self.inner.filter_by_id(id)
                }

                fn filter_by_ref_id(&'a self, ref_id: isize) -> Option<&'a simd_json::BorrowedValue<'a>> {
                    self.inner.filter_by_ref_id(ref_id)
                }

                fn filter_by_node_type(&'a self, node_type: &str) -> Vec<&'a simd_json::BorrowedValue<'a>> {
                    self.inner.filter_by_node_type(node_type)
                }

                fn children_ids(&self) -> Vec<isize> {
                    self.inner.children_ids()
                }

                fn is_string(&self, value: &str) -> bool {
                    self.inner.is_string(value)
                }

                fn is_number(&self, value: isize) -> bool {
                    self.inner.is_number(value)
                }

                fn check_key<T>(&self, key: &str, check: T) -> bool
                where
                    T: FnOnce(&simd_json::BorrowedValue<'_>) -> bool,
                {
                    self.inner.check_key(key, check)
                }

                fn check_chain<const N: usize, T>(&self, chain: [&str; N], check: T) -> bool
                where
                    T: FnOnce(&simd_json::BorrowedValue<'_>) -> bool,
                {
                    self.inner.check_chain(chain, check)
                }

                fn get_key(&'a self, key: &str) -> Option<&'a simd_json::BorrowedValue<'a>> {
                    self.inner.get_key(key)
                }

                fn get_chain<const N: usize>(&'a self, chain: [&str; N]) -> Option<&'a simd_json::BorrowedValue<'a>> {
                    self.inner.get_chain(chain)
                }
            }


            impl<'a> FromBorrowedValue<'a> for [< Zc $name >]<'a> {
                fn from_borrowed_value (value: &'a simd_json::BorrowedValue<'a>) -> Self

                {
                    Self { inner: value }
                }
            }
        }
    };
    (
        @enum @simple $v:vis $name:ident {
            $(
                $variant:ident,
            )*
        }
    ) => {
        paste::paste! {
            $v enum [< Zc $name >] {
                $(
                    [< Zc $variant >],
                )*
            }

            impl<'a> FromBorrowedValue<'a> for [< Zc $name >] {
                fn from_borrowed_value (value: &'a simd_json::BorrowedValue<'a>) -> Self
                {
                    let this = value.as_str().expect("Enum encoded as a string").to_lowercase();
                    match this.as_str() {
                        $(
                            stringify!([< $variant:lower >]) => Self::[< Zc $variant >],
                        )*
                        _ => unreachable!("Impossible for {} Variant reached:  `{}`", stringify!([< Zc $name >]), this),
                    }
                }
            }
        }
    };
    (
        @enum @with_value $v:vis $name:ident {
            $(
                $variant:ident,
            )*
        }
    ) => {
        paste::paste! {
            $v enum [< Zc $name >]<'a> {
                $(
                    [< Zc $variant >]([< Zc $variant >]<'a> ),
                )*
            }

            impl<'a> FromBorrowedValue<'a> for [< Zc $name >]<'a> {
                fn from_borrowed_value(value: &'a simd_json::BorrowedValue<'a>) -> Self
                {
                    let this = value
                        .as_object()
                        .expect("Enum encoded as a string")
                        .get("nodeType")
                        .expect("Complex enum must have nodeType")
                        .as_str()
                        .expect("NodeType is always a string");
                    match this {
                        $(
                            stringify!($variant) => Self::[< Zc $variant >]([< Zc $variant >]::from_borrowed_value(value)),
                        )*
                        _ => unreachable!("Impossible for {} Variant reached:  `{}`", stringify!([< Zc $name >]), value),
                    }
                }
            }
        }
    };
    (
        @enum $v:vis $name:ident {
            $class:ident:
            $(
                $variant:ident,
            )*
        }
    ) => {
        zc_abstract!(@enum @$class $v $name {
            $(
                $variant,
            )*
        });
    };
    ($(
        $v:vis $class:ident $name:ident {$($rest:tt)*}
    )*) => {
        $(
            zc_abstract!(@$class $v $name {$($rest)*});
        )*
    };
}

#[macro_export]
macro_rules! zc_wrapper {
    ($(
        $name:ident $(<$($inner:ident),*>)?
    ),*) => {
        paste::paste!{
            $(
                #[derive(Debug, Clone, Copy)]
                pub struct $name<'a $($(, $inner)*)?> {
                    inner: &'a BorrowedValue<'a>,
                    $(
                        $(
                            [< _phantom_ $inner:lower >]: std::marker::PhantomData<$inner>,
                        )*
                    )?
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! zc_wrap {
    ($(
        $name:ident $(<$($inner:ident),*>)?
    ),*) => {
        paste::paste!{
            $(
                impl<'a $($(, $inner )*)?> IntoZcType<$name<'a $($(, $inner)*)?>> for &'a BorrowedValue<'a> {
                    fn zc_type(&self) -> $name<'a $($(, $inner)*)?> {
                        $name {
                            inner: self,
                            $(
                                $(
                                    [< _phantom_ $inner:lower >]: std::marker::PhantomData,
                                )*
                            )?
                        }
                    }
                }


                impl<'a $($(, $inner)*)?> FromBorrowedValue<'a> for $name<'a $($(, $inner)*)?> {
                    fn from_borrowed_value(value: &'a simd_json::BorrowedValue<'a>) -> Self
                     {
                        $name {
                            inner: value,
                            $(
                                $(
                                    [< _phantom_ $inner:lower >]: std::marker::PhantomData,
                                )*
                            )?
                        }
                    }
                }
            )*
        }
    };
}
