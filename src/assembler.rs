use super::symboltable::SymbolTable;
use crate::parser::InstructionType;
use super::parser::Parser;

pub struct Assembler {
    symbol_table: SymbolTable,
}

impl Assembler {
    pub fn new()->Assembler{
        let symbol_table = SymbolTable::new();
        return Assembler { symbol_table: symbol_table };
    }

    fn first_pass(&mut self,file_path: String) {
        let mut new_parser = Parser::new(file_path);
        let mut curr_address = 0;
        while new_parser.has_more_instructions() {
            new_parser.advance();
            let instr_type = new_parser.instruction_info.instruction_type.clone();
            if instr_type == InstructionType::AInstruction
                || instr_type == InstructionType::CInstruction
            {
                curr_address += 1;
            } else if instr_type == InstructionType::LInstruction {
                self.symbol_table.add_entry(new_parser.instruction_info.symbol.clone(),curr_address);
            }
        }
    }

    pub fn assemble(&mut self,file_path: String) {
        self.first_pass(file_path)
    }
}
