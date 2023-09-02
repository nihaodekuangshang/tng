use std::env::args;
use tools::get_num;
mod Evaluator;
mod Lexer;
mod Parser;

mod tools;

fn main() {
    let mut arg: Vec<String> = args().collect();
    let arg_len = arg.len();
    let code = arg.get_mut(1).unwrap();
    if arg_len > 2 {
        println!("请确保您只输入一个参数");
    } else if arg_len != 2 {
        println!("请确保您输入了参数");
    }
    let ex = Parser::expr(&code);
    print!(".intel_syntax noprefix\n");
    print!(".globl main\n");
    print!("main:\n");
    Evaluator::eval(&ex);
    println!("  pop rax\n");
    print!("    ret\n");
}
