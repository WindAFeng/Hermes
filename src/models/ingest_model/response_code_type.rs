use serde::{Serialize, Serializer};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ResponseCodeType {
    Success = 0,
    NotFoundKey = 1,
    NotFoundColumn = 2,
    NotFoundDatabase = 3,
    ArgNotFound = 4,
    BadRequest = 5
}

impl Serialize for ResponseCodeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 直接序列化为 u8 数字
        serializer.serialize_u8(*self as u8)
    }
}