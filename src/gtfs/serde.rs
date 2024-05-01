pub mod serialisation {
    use serde::{ser, Serializer};

    use crate::gtfs::gtfs_types::ColourCode;

    macro_rules! create_serde_try_into_serialiser {
        ($T:ty, $serialize: ident) => (
            impl serde::Serialize for $T {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                    macro_rules! serde_call {
                        ($self:ident, $F:ident) => {
                            self.try_into().map_err(|e| ser::Error::custom(e)).and_then(|v| $self.$F(v))
                        };
                    }

                    serde_call!(serializer, $serialize)
                }
            }
        )
    }

    create_serde_try_into_serialiser!(ColourCode, serialize_u32);
}

pub mod deserialisation {
    use std::marker::PhantomData;

    use crate::gtfs::gtfs_types::ColourCode;

    struct GTFSVisitor<T>(PhantomData<T>);

    macro_rules! visit_integer_fn {
        ($name:ident: $T:ty, $unexpected:path, $unexpected_str:tt) => (
            fn $name<E: serde::de::Error>(self, v: $T) -> Result<Self::Value, E> {
                v.try_into().map_err(|_| E::invalid_value($unexpected(v.into()), &$unexpected_str))
            }
        )
    }

    macro_rules! create_serde_int_deserialiser {
        ($gtfs_type:ty, $expecting_str:tt, $unexpected_str:tt, $compact:ident) => (
            impl<'de> serde::Deserialize<'de> for $gtfs_type {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where D: serde::Deserializer<'de>
                {
                    macro_rules! call_deserializer {
                        ($self:ident, $F:ident) => {
                            $self.$F(GTFSVisitor::<$gtfs_type>(PhantomData))
                        };
                    }

                    if deserializer.is_human_readable() {
                        // to support json and others, visit any
                        deserializer.deserialize_any(GTFSVisitor::<$gtfs_type>(PhantomData))
                    } else {
                        // hint for more compact that we expect an u64
                        call_deserializer!(deserializer, $compact)
                    }
                }
            }

            impl<'de> serde::de::Visitor<'de> for GTFSVisitor<$gtfs_type> {
                type Value = $gtfs_type;

                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    formatter.write_str($expecting_str)
                }

                visit_integer_fn!(visit_i8: i8, serde::de::Unexpected::Signed, $unexpected_str);
                visit_integer_fn!(visit_i16: i16, serde::de::Unexpected::Signed, $unexpected_str);
                visit_integer_fn!(visit_i32: i32, serde::de::Unexpected::Signed, $unexpected_str);
                visit_integer_fn!(visit_i64: i64, serde::de::Unexpected::Signed, $unexpected_str);

                visit_integer_fn!(visit_u8: u8, serde::de::Unexpected::Unsigned, $unexpected_str);
                visit_integer_fn!(visit_u16: u16, serde::de::Unexpected::Unsigned, $unexpected_str);
                visit_integer_fn!(visit_u32: u32, serde::de::Unexpected::Unsigned, $unexpected_str);
                visit_integer_fn!(visit_u64: u64, serde::de::Unexpected::Unsigned, $unexpected_str);

                fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                    v.parse().map_err(|_| E::invalid_value(serde::de::Unexpected::Str(v), &$unexpected_str))
                }
            }
        )
}

    create_serde_int_deserialiser!(ColourCode, "a colour code as an integer or string", "colour code string", deserialize_u32);
}

#[cfg(test)]
mod serde_tests {
    use serde_test::{assert_ser_tokens, Token};

    use crate::gtfs::gtfs_types::ColourCode;

    #[test]
    fn test_serialisation() {
        let colour_code = ColourCode(0xDEADBEEF);
        assert_ser_tokens(&colour_code, &[Token::U32(0xDEADBEEF)])
    }
}