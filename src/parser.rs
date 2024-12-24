pub struct Parser {
    /*
     * so we want to do what
     * we want to take in each token and determine what it is
     * is it id? is it value? is it operator? is it keyword? is it delimiter?
     * 
     * but then what? 
     *
     * LR(1) parsing step
     */

    tokens: Vec<LexerToken>
}

impl Parser {
    fn process_token(&mut self, token: &String) {
        let chars = token.chars();
        for i in 0..chars.len() {      
            let j = i+1;

            let cur_enum: Option<LexerToken> = None;

            if i != chars.len()-2 {
                
            } else {

            }
        }
    }

    fn str_to_lex(str: &String) {
        match str {
            "i32":  LexerToken::Keyword,
            "u32":  LexerToken::Keyword,
            "bool": LexerToken::Keyword,
            "u1":   LexerToken::Keyword,
            "+":    LexerToken::Op,
            "-":    LexerToken::Op,
            "*":    LexerToken::Op,
            "/":    LexerToken::Op,
            "^":    LexerToken::Op,
            "let":  LexerToken::Op,
            "{":    LexerToken::Delimiter,
            "}":    LexerToken::Delimiter,
            ";":    LexerToken::Delimiter,
            _:      LexerToken::Value,
        }
    }
}

pub enum LexerToken {
    Value,
    Op,
    Keyword,
    Delimiter,
}
