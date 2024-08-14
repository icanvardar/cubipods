use cubipods::Lexer;

fn main() {
    let lex = Lexer::new(&[60, 80]);
    println!("{:?}", lex);
}
