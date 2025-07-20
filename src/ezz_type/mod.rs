#[derive(Debug, PartialEq, Clone)]
pub enum EzzType {
    I32,
    U32,
    F32,
    Void,
    Type,
    Identifier,
    Value,
    Str,
    Char,
    U1,
    Ptr,
}

pub fn str_to_type(st: &'static str) -> EzzType {
    match st {
        "i32" => EzzType::I32,
        "u32" => EzzType::U32,
        "f32" => EzzType::F32,
        "fn" => EzzType::Void,
        "str" => EzzType::Str,
        "char" => EzzType::Char,
        "u1" => EzzType::U1,
        "$" => EzzType::Ptr,
        _ => EzzType::Identifier,
    }
}
