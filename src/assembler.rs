use super::lexer;
use super::parser::Parser;

fn first_pass(file_path:String){
    let mut new_parser = Parser::new(file_path);
    while new_parser.has_more_instructions(){
        new_parser.advance()
    }
}

pub fn assemble(file_path:String){
    first_pass(file_path)
}