#[derive(Debug, PartialEq, Clone)]
pub struct Parser<'a>{
    /*
     * so we want to do what
     * we want to take in each token and determine what it is
     * is it id? is it value? is it operator? is it keyword? is it delimiter?
     * 
     * but then what? 
     *
     * LR(1) parsing step
     */

    pub lex_tokens:  Vec<LexerToken<'a>>,
    pub parse_stack: Vec<ParserToken<'a>>
}

impl<'a> Parser<'_> {
    pub fn new<'b>() -> Parser<'b> {
        Parser { lex_tokens: vec![], parse_stack: vec![] }
    }

    pub fn parse_lexes(&mut self) {
        for x in &self.lex_tokens {
            self.parse_stack.push(ParserToken {parse_type: ParserTokenType::Identifier, literal: "x" } );
        }
        //  
    }

    pub fn shift() {
    }

    pub fn reduce() -> Result<&'static str, &'static str> {
        // we take a token and its corresponding parser token type
        // and then we see if it can be specified
        // e.g "i32" value -> type
        // e.g "mn" value -> identifier
        // e.g "500" value -> number
        //
        // once it stops reducing, we shift
        Ok("Reduced TODO")
    }

    pub fn lex_tokens(&mut self, tokens: &Vec<String>) {
        for i in 0..tokens.len() {
            let cur_token: &'static str = tokens[i].clone().leak();
                                   
            self.lex_tokens.push(LexerToken { lexer_type: Self::str_to_lex(&cur_token), literal: &cur_token} );
        }
    }

    pub fn str_to_lex(str: &str) -> LexerTokenType {
        match str {
            "i32" => LexerTokenType::Keyword,
            "u32" => LexerTokenType::Keyword,
            "bool" => LexerTokenType::Keyword,
            "u1"=>LexerTokenType::Keyword,
            "str"=>LexerTokenType::Keyword,
            "chr"=>LexerTokenType::Keyword,
            "u8"=>LexerTokenType::Keyword,
            "let"=>LexerTokenType::Keyword,
            "mut"=>LexerTokenType::Keyword,
            "+"=>LexerTokenType::Op,
            "-"=>LexerTokenType::Op,
            "*"=>LexerTokenType::Op,
            "/"=>LexerTokenType::Op,
            "^"=>LexerTokenType::Op,
            "let"=>LexerTokenType::Op,
            "{"=>LexerTokenType::Delimiter,
            "}"=>LexerTokenType::Delimiter,
            ";"=>LexerTokenType::Delimiter,
            _=>LexerTokenType::Value,
        }
    }
    
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserToken<'a> {
    parse_type: ParserTokenType,
    literal: &'a str,
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
pub struct LexerToken<'a> {
    lexer_type: LexerTokenType,
    literal: &'a str,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LexerTokenType {
    Value,
    Op,
    Keyword,
    Delimiter,
}
