use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer, Visitor};

macro_rules! primary_gen_deser {
    ($($n:ident,$t:ty,$f:ident),*) => {

        pub enum PrimaryType {
            $(
            $n($t),
            )*
        }

        impl<'de> Deserialize<'de> for PrimaryType {

            fn deserialize<D>(deserializer: D) -> Result<PrimaryType, D::Error>
            where
                D: Deserializer<'de>,
            {
                use std::fmt;
                
                struct PrimaryTypeVisitor;
    
                impl<'de> Visitor<'de> for PrimaryTypeVisitor {
                    type Value = PrimaryType;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("primary types")
                    }

                    $(
                    fn $f<E>(self, value: $t) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Ok(PrimaryType::$n(value))
                    }
                    )*

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Ok(PrimaryType::String(value.to_string()))
                    }
                }

                deserializer.deserialize_any(PrimaryTypeVisitor)
            }
        }

        impl Into<String> for PrimaryType {
            fn into(self) -> String {
                match self {
                    $(
                    PrimaryType::$n(v) => format!("{}", v),
                    )*
                }
            }
        }
    };
}


primary_gen_deser!(
    Bool,bool,visit_bool,
    I8, i8, visit_i8,
    U8, u8, visit_u8,
    I16, i16, visit_i16,
    U16, u16, visit_u16,
    I32, i32, visit_i32,
    U32, u32, visit_u32,
    I64, i64, visit_i64,
    U64, u64, visit_u64,
    Float32, f32, visit_f32,
    Float64, f64, visit_f64,
    String,  String, visit_string   // for `&str`
);


