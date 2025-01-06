/// Macro for creating a numbered enum and associated From impls
///
/// This macro generates an enum and implements [`From`] and [`TryFrom`] for converting between the enum and the underlying integer type. It also generates an error type that can be used to represent errors when converting from the integer type to the enum.
///
/// # Example
///
/// ```rust,ignore
/// numbered_enum! {
///     /// Documentation for the enum
///     /// This will be included in the generated enum
///     #[repr(u8)]
///     pub enum MyEnum {
///         A = 0,
///         B = 1,
///         C = 2,
///     }
///
///     /// Documentation for the error type
///     /// This will be included in the generated error type
///     #[error("thiserror message, bad id: `{0}`")]
///     etype MyEnumError;
/// }
/// ```
///
macro_rules! numbered_enum {
    {
        $(#[doc = $enumdoc:expr])*
        #[repr($enum_type:ty)]
        $enumvis:vis enum $name:ident {
            $(
                $(#[doc = $vardoc:expr])*
                $variant:ident = $value:expr,
            )+
        }

        $(#[doc = $errdoc:expr])*
        #[error($emsg:literal)]
        etype $etype:ident;
    } => {
        $(#[doc = $enumdoc])?
        #[repr($enum_type)]
        #[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        $enumvis enum $name {

            $(
                $(#[doc = $vardoc])*
                $variant = $value,
            )*
        }

        impl From<$name> for $enum_type {
            fn from(value: $name) -> Self {
                value as $enum_type
            }
        }

        $(#[doc = $errdoc])?
        #[derive(Debug, thiserror::Error)]
        #[error($emsg)]
        $enumvis struct $etype($enumvis $enum_type);

        impl TryFrom<$enum_type> for $name {
            type Error = $etype;

            fn try_from(value: $enum_type) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok($name::$variant),)*
                    _ => Err($etype(value)),
                }
            }
        }
    };
}

pub(crate) use numbered_enum;
