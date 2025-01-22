/*
 *
 *
 *  what kinds of things do we need
 *
 *  we need the original literal,
 *      type,
 *      variables,
 *      calls,
 *
 *
 *  okay so what is a type?
 *
 *      it is a piece of info helping us know what a specific Value is.
 *          a specific Value might be a string, and from it being added to an i32, we know
 *          something is off here.
 *
 *
 *  what is a variable?
 *      
 *      it is a name we attribute to what comes out as a Value
 *
 *
 *
 *  what is a call?
 *      
 *      it is a name we attribute to using a function
 *      also like a go-to, or a copy paste
 *
 */

pub struct CompilerType {
    type_info: &'static str,
}

pub struct CompilerVar {
    ctype: CompilerType,
    literal: &'static str,
}
impl CompilerObject for CompilerVar {
    pub fn get_type() -> &'static str {
        self.ctype.type_info
    }

    pub fn get_literal() -> &'static str {
        self.literal
    }
}

pub struct CompilerCall {
    ctype: &'static str,
    literal: &'static str,
}
impl CompilerObject for CompilerCall {
    pub fn get_type() -> &'static str {
        self.ctype.type_info
    }

    pub fn get_literal() -> &'static str {
        self.literal
    }
}

pub trait CompilerObject {
    pub fn get_type(&self) -> &'static str;
    pub fn get_literal(&self) -> &'static str;
}
