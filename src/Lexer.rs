#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    Num(usize),
    Op(char),

    Eof,
}

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut tokens = Vec::new();
        let len = input.len();
        let mut index: usize = 0;
        while index < len {
            match input.chars().nth(index) {
                Some(ch) => {
                    if ch.is_digit(10) {
                        let (num, num_len) = read_num(&input[index..]).unwrap();
                        index = index + num_len;
                        tokens.push(Token::Num(num));
                        continue;
                    } else if !ch.is_ascii_whitespace() {
                        tokens.push(Token::Op(ch));
                    }
                }
                None => break,
            }
            index = index + 1;
        }
        tokens.reverse();

        Self { tokens }
    }
    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }
    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

fn read_num(input: &str) -> Option<(usize, usize)> {
    let len = input.len();
    let mut index = 0;
    let mut num = 0;
    while index < len {
        match input.chars().nth(index) {
            Some(ch) => {
                if ch.is_digit(10) {
                    num = num * 10 + ch.to_digit(10)? as usize;
                } else {
                    break;
                }
            }
            None => break,
        }
        index = index + 1;
    }
    Some((num, index))
}
