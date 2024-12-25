#[derive(Debug, PartialEq, Clone)]
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

    pub lex_tokens: Vec<LexerToken>,
    pub parse_stack: Vec<ParserToken>
}

impl Parser {
    pub fn new() -> Parser {
        Parser { lex_tokens: vec![] }
    }

    pub fn parse_lexes(&mut self) {
        self.parse_stack.append();
        //  
    }

    pub fn shift() {
    }

    pub fn reduce() -> Result<> {
        // we take a token and its corresponding parser token type
        // and then we see if it can be specified
        // e.g "i32" value -> type
        // e.g "mn" value -> identifier
        // e.g "500" value -> number
        //
        // once it stops reducing, we shift
    }

    pub fn lex_tokens(&mut self, tokens: &Vec<String>) {
        for i in 0..tokens.len() {
            let cur_token = &lex_tokens[i];
                                   
            self.lex_tokens.push(Self::str_to_lex(&cur_token));
        }
    }

    pub fn str_to_lex(str: &String) -> LexerToken {
        match str.as_str() {
            "i32" => LexerToken::Keyword,
            "u32" => LexerToken::Keyword,
            "bool" => LexerToken::Keyword,
            "u1"=>LexerToken::Keyword,
            "str"=>LexerToken::Keyword,
            "chr"=>LexerToken::Keyword,
            "u8"=>LexerToken::Keyword,
            "let"=>LexerToken::Keyword,
            "mut"=>LexerToken::Keyword,
            "+"=>LexerToken::Op,
            "-"=>LexerToken::Op,
            "*"=>LexerToken::Op,
            "/"=>LexerToken::Op,
            "^"=>LexerToken::Op,
            "let"=>LexerToken::Op,
            "{"=>LexerToken::Delimiter,
            "}"=>LexerToken::Delimiter,
            ";"=>LexerToken::Delimiter,
            _=>LexerToken::Value,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserToken {
    parse_type: ParserTokenType,
    literal: &str,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserTokenType {
    Value,
    Identifier,
    Op,
    Keyword,
    Type,
    Int32,
    UInt32,
    Boolean,
    Arr,
    Reserved,
    Delimiter
}

#[derive(Debug, PartialEq, Clone)]
pub enum LexerToken {
    Value,
    Op,
    Keyword,
    Delimiter,
}
