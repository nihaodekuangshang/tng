use std::env::args;

fn main() {
    let arg: Vec<String> = args().collect();
    let num = arg.len();
    if num > 2 {
        println!("请确保您只输入一个参数");
    } else if num != 2 {
        println!("请确保您输入了参数");
    }
    print!(".intel_syntax noprefix\n");
    print!(".globl main\n");
    print!("main:\n");
    print!("  mov rax, {}\n", arg.get(1).unwrap());
    print!("  ret\n");
}
