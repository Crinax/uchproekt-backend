use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum FieldType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "discount")]
    Discount,
    #[serde(rename = "unknown")]
    #[default]
    Unknown,
}

macro_rules! impl_field_type {
    ($($impl_type:ty),*) => {
        $(
            impl From<$impl_type> for FieldType {
                fn from(value: $impl_type) -> Self {
                    match value {
                      1 => FieldType::String,
                      2 => FieldType::Integer,
                      3 => FieldType::Discount,
                      _ => FieldType::Unknown
                    }
                }
            }

            impl From<FieldType> for $impl_type {
                fn from(value: FieldType) -> Self {
                    match value {
                      FieldType::String => 1,
                      FieldType::Integer => 2,
                      FieldType::Discount => 3,
                      FieldType::Unknown => 0
                    }
                }
            }
        )*
    };
}

impl From<&str> for FieldType {
    fn from(value: &str) -> Self {
        match value {
            "string" => FieldType::String,
            "integer" => FieldType::Integer,
            "discount" => FieldType::Discount,
            _ => FieldType::Unknown,
        }
    }
}

impl_field_type!(i8, i32, i64, i128, u8, u32, u64, u128);
