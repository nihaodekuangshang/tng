use std::env::args;
use token::Token;
use tools::get_num;
mod token;
mod tools;

fn main() {
    let mut arg: Vec<String> = args().collect();
    let arg_len = arg.len();
    let code = arg.get_mut(1).unwrap();
    let tokens = tokenize(&code[..]);
    if arg_len > 2 {
        println!("请确保您只输入一个参数");
    } else if arg_len != 2 {
        println!("请确保您输入了参数");
    }
    print!(".intel_syntax noprefix\n");
    print!(".globl main\n");
    print!("main:\n");
    let tok = tokens.get(0).unwrap();
    print!("    mov rax,{}\n", tok.expect_num().unwrap());
    let mut index = 1;
    let len = tokens.len();
    while index < tokens.len() {
        match tokens.get(index) {
            Some(tok) => {
                if tok.expect('+') {
                    index = index + 1;
                    if index >= len {
                        break;
                    }
                    let tok = tokens.get(index).unwrap();
                    print!("    add rax,{}\n", tok.expect_num().unwrap());
                } else if tok.expect('-') {
                    index = index + 1;
                    if index >= len {
                        break;
                    }
                    let tok = tokens.get(index).unwrap();
                    print!("    sub rax,{}\n", tok.expect_num().unwrap());
                } else if tok.kind == token::TokenKind::TkIllegal {
                    eprintln!("您输入了非法字符");
                }
            }
            None => {
                eprintln!("获取token出错,序号：{}", index)
            }
        }
        index = index + 1;
    }
    print!("    ret\n");
}
fn tokenize<'b, 'a: 'b>(code_s: &'b str) -> Vec<Token<'_>> {
    let mut tokens = Vec::<Token>::new();
    let len = code_s.len();
    let mut index = 0usize;
    while index < len {
        let pc = code_s.chars().nth(index).unwrap();
        if pc.is_digit(10) {
            tokens.push(Token::new(token::TokenKind::TkNum, &code_s[index..]));
            index = index + tokens.last().unwrap().str.len();
            continue;
        } else if pc == '+' || pc == '-' {
            tokens.push(Token::new(token::TokenKind::TkReserved, &code_s[index..]));
            index = index + tokens.last().unwrap().str.len();
            continue;
        } else if pc.is_whitespace() {
            index = index + 1;
            continue;
        } else {
            tokens.push(Token::new(token::TokenKind::TkIllegal, &code_s[index..]));
            index = index + tokens.last().unwrap().str.len();
            continue;
        }
    }
    tokens.push(Token::new(token::TokenKind::TkEof, code_s));
    tokens
}
