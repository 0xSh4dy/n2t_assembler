use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug,PartialEq)]
pub enum ExpressionType {
    Number,    // Number constant
    Symbol,    // Symbol constant, such as LOOP / END etc.
    Operation, // Operation constant, such as + , = , - ,etc.
    Error,
}

fn get_word_regex() -> Regex {
    let number_re = r"\d+";
    let symbol_start_re = r"\w_.$:";
    let symbol_re = "[".to_string() + symbol_start_re + "][" + symbol_start_re + r"\d]*";
    let operation_re = r"[=;()@+\-&|!]";
    let word_re = Regex::new(&format!("({}|{}|{})", number_re, symbol_re, operation_re)).unwrap();
    return word_re;
}

fn replace_comment(line: &str) -> String {
    let comment = Regex::new(r"//.*$").unwrap();
    return comment.replace_all(line, "").to_string();
}

fn is_number(word: &String) -> bool {
    let number_re = Regex::new(r"\d+").unwrap();
    return number_re.is_match(word.as_str());
}

fn is_symbol(word: &String) -> bool {
    let symbol_start_re = r"\w_.$:";
    let symbol_re_str = "[".to_string() + symbol_start_re + "][" + symbol_start_re + r"\d]*";
    let symbol_re = Regex::new(symbol_re_str.as_str()).unwrap();
    return symbol_re.is_match(word.as_str());
}

fn is_operation(word: &String) -> bool {
    let operation_re = Regex::new(r"[=;()@+\-&|!]").unwrap();
    return operation_re.is_match(word.as_str());
}

fn get_token_for_word(word: String) -> (ExpressionType, String) {
    if is_number(&word) {
        return (ExpressionType::Number, word);
    } else if is_symbol(&word) {
        return (ExpressionType::Symbol, word);
    } else if is_operation(&word) {
        return (ExpressionType::Operation, word);
    } else {
        return (ExpressionType::Error, word);
    }
}

fn tokenize_line(line: String) -> Vec<(ExpressionType, String)> {
    let text = replace_comment(line.as_str());
    let word_regex = get_word_regex();
    let mut tokens: Vec<(ExpressionType, String)> = Vec::new();
    let words: Vec<_> = word_regex.find_iter(text.as_str()).collect();
    if words.len() != 0 {
        for word in words {
            let token = get_token_for_word(String::from(word.as_str()));
            tokens.push(token);
        }
    }
    return tokens;
}

fn tokenize(file_path: String) -> Result<Vec<Vec<(ExpressionType, String)>>, std::io::Error> {
    let file: File = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut tokens: Vec<Vec<(ExpressionType, String)>> = Vec::new();
    for line in reader.lines() {
        let line_str: String = line?;
        let line_tokens = tokenize_line(line_str);
        tokens.push(line_tokens);
    }
    Ok(tokens)
}

fn get_tokens(file_path: String) -> Result<Vec<Vec<(ExpressionType, String)>>, std::io::Error> {
    let tokens = tokenize(file_path);
    return tokens;
}
pub fn run_lexer(file_path: String) {
    let mut tokens: Vec<Vec<(ExpressionType, String)>> = get_tokens(file_path).unwrap();
    loop {
        if tokens.len() == 0 {
            break;
        }
        let mut curr_instr_tokens = tokens.remove(0);
        let mut curr_token: (ExpressionType, String);
        if curr_instr_tokens.len() != 0 {
            curr_token = curr_instr_tokens.remove(0);
        } else {
            curr_token = (ExpressionType::Error, "0".to_string());
        }
        let (token, val) = curr_token;
        if token == ExpressionType::Operation{
            if val == "@"{
                // A instruction
            }
            else if val == "("{
                // L instruction
            }
        }
        else{
            // C instruction
        }
    }
}
