use super::lexer::{ExpressionType, Lexer};
pub struct Parser {
    pub new_lexer: Lexer,
    pub instruction_info: InstructionInfo,
}

#[derive(PartialEq, Clone)]
pub enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
    InvalidInstruction,
}

pub struct InstructionInfo {
    pub instruction_type: InstructionType,
    pub symbol: String,
    dest: String,
    comp: String,
    jump: String,
}

fn init_instruction_info() -> InstructionInfo {
    InstructionInfo {
        instruction_type: InstructionType::InvalidInstruction,
        symbol: "".to_string(),
        dest: "".to_string(),
        comp: "".to_string(),
        jump: "".to_string(),
    }
}

impl Parser {
    pub fn new(file_path: String) -> Parser {
        Parser {
            new_lexer: Lexer::new(file_path),
            instruction_info: init_instruction_info(),
        }
    }

    fn a_instruction(&mut self) {
        self.instruction_info.instruction_type = InstructionType::AInstruction;
        (_, self.instruction_info.symbol) = self.new_lexer.next_token();
    }

    fn l_instruction(&mut self) {
        self.instruction_info.instruction_type = InstructionType::LInstruction;
        (_, self.instruction_info.symbol) = self.new_lexer.next_token();
    }

    fn c_instruction(&mut self, token: ExpressionType, value: String) {
        self.instruction_info.instruction_type = InstructionType::CInstruction;
        let (comp_tok, comp_val) = self.get_dest(token, value);
        self.get_comp(comp_tok, comp_val);
        self.get_jump();
    }

    fn get_comp(&mut self, token: ExpressionType, value: String) {
        if token == ExpressionType::Operation && (value == "-" || value == "!") {
            let (_, val2) = self.new_lexer.next_token();
            let mut temp_str = String::new();
            temp_str.push_str(value.as_str());
            temp_str.push_str(val2.as_str());
            self.instruction_info.comp = temp_str.clone();
        } else if token == ExpressionType::Number || token == ExpressionType::Symbol {
            self.instruction_info.comp = value;
            let (tok2, val2) = self.new_lexer.peek_token();
            if tok2 == ExpressionType::Operation && val2 != ";" {
                self.new_lexer.next_token();
                let (_, val3) = self.new_lexer.next_token();
                self.instruction_info.comp.push_str(val2.as_str());
                self.instruction_info.comp.push_str(val3.as_str());
            }
        }
    }

    fn get_jump(&mut self) {
        let (token, value) = self.new_lexer.next_token();
        if token == ExpressionType::Operation && value == ";" {
            let (_, jump_val) = self.new_lexer.next_token();
            self.instruction_info.jump = jump_val.clone();
        }
    }

    fn get_dest(&mut self, token: ExpressionType, value: String) -> (ExpressionType, String) {
        // Gets the 'dest' part of the instruction, if any.
        // returns the first token of the comp part | dest=comp
        let (tok2, val2) = self.new_lexer.peek_token();
        if tok2 == ExpressionType::Operation && val2 == "=" {
            self.new_lexer.next_token();
            self.instruction_info.dest = value.clone();
            return self.new_lexer.next_token();
        }
        return (token, value);
    }
    pub fn has_more_instructions(&self) -> bool {
        return self.new_lexer.has_more_instructions();
    }

    pub fn advance(&mut self) {
        self.instruction_info = init_instruction_info();
        self.new_lexer.next_instruction();
        let (token, val) = self.new_lexer.curr_token.clone();
        if token == ExpressionType::Operation {
            if val == "@" {
                self.a_instruction()
            } else if val == "(" {
                self.l_instruction()
            }
        } else if token != ExpressionType::Error {
            self.c_instruction(token, val);
        }
    }
}
