#[macro_export]
macro_rules! enum_refs {
    (
        #[derive_owned($($derive_owned:path),*)]
        #[derive($($derive:path),*)]
        $(#[$additional:meta])?
        $v:vis enum $name:ident {
            $(
                $variant:ident($variant_type:ty),
            )*
        }
    ) => {
        #[derive($($derive_owned,)* $($derive),*)]
        $(#[$additional])?
        $v enum $name {
            $(
                $variant($variant_type),
            )*
        }

        $(
            crate::convert_enums! { $name, $variant }
        )*

        paste::paste! {
            #[derive($($derive),*)]
            $v enum [< $name Ref >]<'a> {
                $(
                    $variant(&'a $variant_type),
                )*
            }

            $(
                crate::convert_refs_enums! { [< $name Ref >], $variant }
            )*

            impl<'a> From<&'a $name> for [< $name Ref >]<'a> {
                fn from(value: &'a $name) -> Self {
                    match value {
                        $(
                            $name::$variant(block) => Self::$variant(block),
                        )*
                    }
                }
            }
        }

    };
}
