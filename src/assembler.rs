use super::lexer;
fn first_pass(file_path:String){
    let curr_address:u16 = 0;
    lexer::run_lexer(file_path);
}

pub fn assemble(file_path:String){
    first_pass(file_path)
}