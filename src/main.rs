mod assembler;
mod lexer;
mod parser;
mod symboltable;
fn main() {
    let mut asm = assembler::Assembler::new();
    asm.assemble("/home/sh4dy/Desktop/nand2tetris/Assembler.hack/tests/programs/Add.asm".to_string());
}
