use std::{env::args, io::Error, str::Chars};

fn main() {
    let mut arg: Vec<String> = args().collect();
    let arg_len = arg.len();
    let code = arg.get_mut(1).unwrap();
    let code_c = code.chars();
    if arg_len > 2 {
        println!("请确保您只输入一个参数");
    } else if arg_len != 2 {
        println!("请确保您输入了参数");
    }
    print!(".intel_syntax noprefix\n");
    print!(".globl main\n");
    print!("main:\n");
    let mut index = 0;
    let mut num = get_num(&code, &mut index).unwrap();
    print!("    mov rax,{}\n", num);
    let len = code.len();
    while index < len {
        let pc = code_c.clone().nth(index).unwrap();
        if pc == '+' {
            index = index + 1;
            if index >= len {
                break;
            }
            num = get_num(&code, &mut index).unwrap();
            print!("    add rax,{}\n", num);
        } else if pc == '-' {
            index = index + 1;
            if index >= len {
                break;
            };
            num = get_num(&code, &mut index).unwrap();
            print!("    sub rax,{}\n", num);
        } else {
            num = get_num(&code, &mut index).unwrap();
            eprint!("不合法的数值 {},{}", index, num);
        }
    }
    print!("    ret\n");
}
fn get_num(str: &String, index: &mut usize) -> Result<usize, Error> {
    let len = str.len();
    let str_c = str.chars();
    let mut ca = str_c.clone().nth(*index).unwrap();
    let mut num = 0usize;
    while ca.is_digit(10) && *index < len {
        num = num * 10 + ca.clone().to_digit(10).unwrap() as usize;
        *index = *index + 1;
        if *index >= len {
            break;
        }
        ca = str_c.clone().nth(*index).unwrap();
    }
    Ok(num)
}
