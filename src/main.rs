#[derive(Debug)]
pub enum Type
{
    Integer,
    Float,
    String,
    Word,
    Symbol,

    Indent,
    Dedent,
    Newline,

    EOF,
}


#[derive(Debug)]
pub struct Position (pub u32, pub u32);


#[derive(Debug)]
pub struct Token
{
    pub ttype: Type,
    pub position: Position,
    pub lexeme: Option<String>,
}


#[derive(Debug)]
pub struct Alias
{
    name: Token,
    id: u32,
}


#[derive(Debug)]
pub struct Segment
{
    name: Token,
    variant: Variant,
    tokens: Vec<Token>,
}


#[derive(Debug)]
pub struct Variant
{
    name: Token,
    rule: Rule,

    tokens: Vec<Token>,
    segments: Vec<Segment>,
    alias: Vec<Alias>,

}


#[derive(Debug)]
pub struct Rule
{
    name: Token,
    variants: Vec<Variant>,
}


#[derive(Debug)]
pub struct Source
{
    lines: Vec<String>,
    tokens: Vec<Token>,
}


#[derive(Debug)]
pub struct Compiler
{
    line: u32,
    token: u32,

    source: Source,
//    ast: Vec<Node>,
}


impl Token
{
    pub fn new (ttype: Type, position: Position, lexeme: Option<String>) -> Token
    {
        Token { ttype, position, lexeme }
    }
}



fn main ()
{
}
