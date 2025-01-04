#[derive(Debug, PartialEq, Clone)]
pub struct Parser<'a>{
    pub parse_stack: Vec<ParserToken<'a>>
}

impl<'a> Parser<'_> {
    pub fn new<'b>() -> Parser<'b> {
        Parser { parse_stack: vec![] }
    }

    // lex is necessary to convert from string into a token
    //  as opposed to the parse which changes tokens into simpler tokens
    pub fn lex(&self, tokens: &Vec<String>) -> Vec<ParserToken> {
        let mut lexes: Vec<ParserToken> = vec![];
        
        for s in tokens {
            lexes.push(self::str_to_lex(s));
        }
    }

    pub fn parse(&mut self, tokens: &Vec<ParserToken>) -> bool {
        
    }

    pub fn shift() {
    }

    pub fn reduce() -> Result<&'static str, &'static str> {
        Ok("Reduced TODO")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserToken<'a> {
    parse_type: ParserTokenType,
    literal: &'a str,
}

// what kinds of things do we really need in parser tokens?
// value op value reduces to value
//
//   ex:  
//          i32 mn $args {
//              put "Hello, World!";
//              let x = 5 + 5;
//              0
//          }
//
//  this whole thing needs to reduce to an acceptable token
//
//  this is why the C expr token exists
//
//  so the whole thing needs to eval to expr
//
//  lets reduce the whole example so far
//
//  Type Reserved Arr Type Reserved Delimiter Id String Id Id Op Value Op Value Value Delimiter
//
//  Function Reserved Delimiter Function Expr Value Delimiter
//  Function Reserved Expr
//  Function
//
//  I think Func can also work as a final value, we just call the main function if it exists
//
//  * = 0+
//  Grammar Rules:
//      Type Id (Type Id Comma)* => Func
//
//      Id Value => Func
//
//      Id Id Op Value => Expr
//
//      Value Op Value => Expr
//
//      Func Id Expr Value => Func
//
//      String => Value
//
//  Let's interpret the ex using these rules and LR(1) parsing
//
//      Shift
//
//      "i32" "mn"
//      type   mn
//      type   id  $args
//      type   id  type id
//      
//      func delimiter
//      func delimiter put
//      func delimiter id "Hello, world!"
//      func delimiter id value
//      func delimiter func
//      func delimiter func let
//      func delimiter func id  x
//      func delimiter func id  id
//      func delimiter func id  id =
//      func delimiter func id  id op
//      func delimiter func id  id op 5
//      func delimiter func id  id op value
//      func delimiter func expr
//      func delimiter func func 0
//      func delimiter func func value
//      func delimiter func func value delimiter
//      func delimiter func func value delimiter
//
//      we halt there on current rules
//
//      we add one more rule:
//          func delimiter func* value delimiter => func
//
//      now we get our last reduce:
//
//      func
//
#[derive(Debug, PartialEq, Clone)]
pub enum ParserTokenType {
    Value,
    Expr,
    Func,
    Id,
    Op,
    Type,
    Arr,
    Reserved,
    Delimiter,
    Comma
}
