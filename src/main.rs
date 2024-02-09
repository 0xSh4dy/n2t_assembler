mod assembler;
mod lexer;
mod parser;
fn main() {
    assembler::assemble("/home/sh4dy/Desktop/nand2tetris/Assembler.hack/tests/programs/Add.asm".to_string());
}
