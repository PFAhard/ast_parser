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
                impl<'a $($(, $inner)*)?> IntoZcType<$name<'a $($(, $inner)*)?>> for &'a BorrowedValue<'a> {
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
                    fn from_borrowed_value(value: &'a BorrowedValue<'a>) -> Self {
                        $name {
                            inner: value,
                            $(
                                $(
                                    [< _phantom_ $inner:lower >]: PhantomData,
                                )*
                            )?
                        }
                    }
                }
            )*
        }
    };
}