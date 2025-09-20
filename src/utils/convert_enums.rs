#[macro_export]
macro_rules! convert_enums {
    ($enum_name:ident, $variant:ident) => {
        impl From<$variant> for $enum_name {
            fn from(value: $variant) -> Self {
                $enum_name::$variant(value)
            }
        }

        impl TryFrom<$enum_name> for $variant {
            type Error = $enum_name;

            fn try_from(value: $enum_name) -> Result<Self, Self::Error> {
                match value {
                    $enum_name::$variant(v) => Ok(v),
                    other => Err(other),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! convert_refs_enums {
    ($enum_name:ident, $variant:ident) => {
        impl<'a> From<&'a $variant> for $enum_name<'a> {
            fn from(value: &'a $variant) -> Self {
                $enum_name::$variant(value)
            }
        }

        impl<'a> TryFrom<$enum_name<'a>> for &'a $variant {
            type Error = $enum_name<'a>;

            fn try_from(value: $enum_name<'a>) -> Result<Self, Self::Error> {
                match value {
                    $enum_name::$variant(v) => Ok(v),
                    other => Err(other),
                }
            }
        }
    };
}
