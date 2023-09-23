use std::env::args;
use std::fs::File;
use std::io::Read;
use tools::get_num;
mod Error;
mod Lexer;
mod tools;

fn main() {
    let mut arg: Vec<String> = args().collect();
    let arg_len = arg.len();
    if arg_len > 2 {
        println!("请确保您只输入一个参数");
    } else if arg_len < 2 {
        run_cmd();
    } else {
        let path = arg.get_mut(1).unwrap();
        let mut code: String = String::new();
        let _ = File::open(path).unwrap().read_to_string(&mut code);
        let mut l = Lexer::Lexer::new(&code);
        l.lexer_tokens();
        println!("{:?}", l);
    }
}
fn run_file(path: &String) {
    let mut code: String = String::new();
    let _ = File::open(path).unwrap().read_to_string(&mut code);
    run(&code);
}
fn run_cmd() {
    let stdin = std::io::stdin();
    loop {
        let mut code: String = String::new();
        print!(">");
        stdin.read_line(&mut code);
        if code == "" {
            break;
        }
        let mut l = Lexer::Lexer::new(&code);
        l.lexer_tokens();
        println!("{:?}", l);
        //run(&code);
    }
}
fn run(code: &str) {
    print!(".intel_syntax noprefix\n");
    print!(".globl main\n");
    print!("main:\n");
    //let ex = Parser::expr(&code);
    //Evaluator::eval(&ex);
    print!("    pop rax\n");
    print!("    ret\n");
}
