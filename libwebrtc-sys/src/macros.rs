/// This macro is specifically for use with the "enum" types that CXX generates
/// for C++ enums.
///
/// C++ enums exposed to rust contain a "repr" field to indicate their type.
/// It's tedious to write out the match expression for each enum variant, so we
/// have this macro.
///
/// It's possible to receive invalid input for any of these enums so we implement
/// the `TryInto` trait ensuring we provide error handling for invalid/unknown input.
///
/// # Example
///
/// // NOTE: Compile fail since it's difficult to doc cxx bridges.
/// ```compile_fail
///
/// use std::convert::TryInto;
///
/// // example bridge
/// #[cxx::bridge]
/// pub mod ffi {
///     #[repr(i32)]
///     enum MyEnum {
///        Foo = 120,
///        Bar = 122
///     }
///
///     unsafe extern "C++" {
///         type MyEnum;
///     }
///
/// }
///
/// primitive_to_cxx_enum!(MyEnum, i32, Foo, Bar);
/// ```
///
#[macro_export]
macro_rules! primitive_to_cxx_enum {
    ($enum:ident, $repr:ty, $($val: ident),*) => {
        impl std::convert::TryFrom<$repr> for $enum {
            type Error = crate::rust_errors::LibWebRTCError;

            fn try_from(value: $repr) -> Result<Self, Self::Error> {
                match value {
                    $(
                        v if v == $enum::$val.repr => Ok($enum::$val),
                    )*
                    _ => Err(crate::rust_errors::LibWebRTCError::ConversionError(
                        format!("{} is not a valid {}", value, stringify!($enum))
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    #[derive(PartialEq, Debug)]
    pub struct WithPrimitives {
        pub repr: i32,
    }

    impl WithPrimitives {
        pub const FIVE: Self = Self { repr: 5 };
        pub const SIX: Self = Self { repr: 6 };
        pub const SEVEN: Self = Self { repr: 7 };
    }

    primitive_to_cxx_enum!(WithPrimitives, i32, FIVE, SIX, SEVEN);

    #[test]
    fn macros() {
        let five = WithPrimitives::try_from(5i32).unwrap();
        assert_eq!(five, WithPrimitives::FIVE);
        assert!(WithPrimitives::try_from(100i32).is_err());
    }
}
