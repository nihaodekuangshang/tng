use crate::Lexer::Lexer;
use crate::Lexer::Token;
use std::fmt::Display;
pub enum Expression {
    Num(usize),
    Cons(char, Vec<Expression>),
}
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(num) => {
                write!(f, "{}", num)
            }
            Self::Cons(op, V) => {
                write!(f, "({}", op)?;
                for ex in V {
                    write!(f, "{} ", ex)?
                }
                write!(f, ")")
            }
        }
    }
}

pub fn expr(input: &str) -> Expression {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0)
}
fn expr_bp(lexer: &mut Lexer, min_bp: usize) -> Expression {
    let token = lexer.next();
    let mut lex = match token {
        Token::Num(num) => Expression::Num(num),
        Token::Op('(') => {
            let lex = expr_bp(lexer, 0);
            assert_eq!(Token::Op(')'), lexer.next());
            lex
        }
        Token::Op(op) => {
            let ((), r_bp) = prefix_binding_power(op);
            let rex = expr_bp(lexer, r_bp);
            Expression::Cons(op, vec![rex])
        }
        t => {
            panic!("不被期望的符号 {:?}", t)
        }
    };
    loop {
        let op = match lexer.peek() {
            Token::Op(op) => op,
            Token::Eof => break,
            Token::Num(num) => panic!(" {}是不被期望的", num),
        };
        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();
            let rex = expr_bp(lexer, r_bp);
            lex = Expression::Cons(op, vec![lex, rex]);
            continue;
        }
        break;
    }
    lex
}
pub fn prefix_binding_power(op: char) -> ((), usize) {
    match op {
        '+' | '-' => ((), 5),
        _ => panic!("{},这是不合法的符号，它不该出现在这里", op),
    }
}
pub fn infix_binding_power(op: char) -> Option<(usize, usize)> {
    match op {
        '+' | '-' => Some((1, 2)),
        '*' | '/' => Some((3, 4)),
        '.' => Some((7, 8)),
        _ => None,
    }
}
