use crate::Parser::Expression;
pub fn eval(ex: &Expression) {
    match ex {
        Expression::Num(num) => print!("push {}\n", num),
        Expression::Cons(op, V) => {
            if V.len() < 2 {
                print!("    push 0\n");
                eval(V.get(0).unwrap());
                print!("    pop rdi\n");
                print!("    pop rax\n");
                match op {
                    '+' => {
                        print!("    add rax,rdi\n");
                        print!("    push rax\n");
                    }
                    '-' => {
                        print!("    sub rax,rdi\n");
                        print!("    push rax\n");
                    }
                    _ => (),
                }
            } else {
                eval(V.get(0).unwrap());
                eval(V.get(1).unwrap());
                print!("    pop rdi\n");
                print!("    pop rax\n");
                match op {
                    '+' => {
                        print!("    add rax,rdi\n");
                        print!("    push rax\n");
                    }
                    '-' => {
                        print!("    sub rax,rdi\n");
                        print!("    push rax\n");
                    }
                    '*' => {
                        print!("    imul rax,rdi\n");
                        print!("    push rax\n");
                    }
                    '/' => {
                        print!("    cqo\n");
                        print!("    idiv rdi\n");
                        print!("    push rax\n");
                    }
                    _ => (),
                }
            }
        }
    }
}
