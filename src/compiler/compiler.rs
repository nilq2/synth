use tokenizer::tokenizer::Source;
use parser::unit::Node;



#[derive(Debug)]
pub struct AST<'a> {
    source: &'a Source<'a>,
    ast: Vec<Node<'a>>,
}


impl<'a> AST<'a> {
    pub fn new (source: &'a Source<'a>, ast: Vec<Node<'a>>) -> AST<'a> {
        AST { source, ast }
    }

    pub fn analyse (&self) {}


    pub fn compile (&self) {
        for node in self.ast.iter() {
            dump_node(&self.source, &node, 1);
            println!("");
        }
    }
}


fn dump_node(source: &Source, node: &Node, dent: usize) {
    for _ in 0..dent { print!("   "); }

    print!("{}", node.variant.name.lexeme.unwrap());

    if node.tokens.len() > 0 {
        print!(" ( ");
    }

    for token in node.tokens.iter() {
        print!("{}[{}], ",
            token.name.lexeme.unwrap(),
            source.tokens.as_ref().unwrap()[token.token].lexeme.unwrap()
        );
    }

    if node.tokens.len() > 0 {
        println!(")");
    } else {
        println!("");
    }

    for child in node.children.iter() {
        dump_node(source, child, dent+1);
    }
}
