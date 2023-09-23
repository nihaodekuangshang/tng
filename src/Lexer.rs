use crate::Error::LuaError;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    MINUS,
    PLUS,
    LF,
    STAR,
    POWER,
    PERCENT,
    POUND,

    // One or two character tokens.
    TILDE,
    TildeEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,
    DotLink,
    SLASH,
    DoubleSlash,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    Eof,
}
#[derive(Clone, Copy, Debug)]
pub enum Literal<'a> {
    Nil,
    Boolean(bool),
    Number(f64),
    Str(&'a str),
    Function,
    Userdata,
    Thread,
    Table,
}
#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    kind: TokenType,
    lexeme: &'a str,
    literal: Literal<'a>,
    line: usize,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}
impl<'a, 'b: 'a> Lexer<'a> {
    pub fn new(input: &'b str) -> Self {
        let mut tokens = Vec::<Token<'a>>::new();
        Self {
            source: input,
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn lexer_tokens(&mut self) {
        while !self.is_end() {
            self.start = self.current;
            println!("lex_tokensa:{}", self.start);
            self.lexer_token();
        }
        self.tokens.push(Token {
            kind: TokenType::Eof,
            lexeme: "",
            literal: Literal::Nil,
            line: self.line,
        });
    }
    fn lexer_token(&mut self) {
        match self.next_char() {
            Some(c) => match c {
                '+' => {
                    self.tokens.push(Token {
                        kind: TokenType::PLUS,
                        lexeme: &self.source[self.start..self.current],
                        literal: Literal::Nil,
                        line: self.line,
                    });
                }
                '-' => match self.peek_char() {
                    Some(next_c) => {
                        if next_c == '-' {
                            self.next_char();
                            if let Some(ano) = self.peek_char() {
                                if ano == '[' {
                                    todo!();
                                }
                            }
                            while let Some(ano) = self.next_char() {
                                if ano == '\n' {
                                    break;
                                }
                            }
                        } else {
                            self.tokens.push(Token {
                                kind: TokenType::MINUS,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        }
                    }
                    None => {}
                },
                '*' => {
                    self.tokens.push(Token {
                        kind: TokenType::STAR,
                        lexeme: &self.source[self.start..self.current],
                        literal: Literal::Nil,
                        line: self.line,
                    });
                }
                '/' => match self.peek_char() {
                    Some(next_c) => {
                        if next_c == '/' {
                            let _ = self.next_char();
                            self.tokens.push(Token {
                                kind: TokenType::DoubleSlash,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        } else {
                            self.tokens.push(Token {
                                kind: TokenType::SLASH,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        }
                    }
                    None => {}
                },

                '^' => {
                    self.tokens.push(Token {
                        kind: TokenType::POWER,
                        lexeme: &self.source[self.start..self.current],
                        literal: Literal::Nil,
                        line: self.line,
                    });
                }
                '=' => match self.peek_char() {
                    Some(next_c) => {
                        if next_c == '=' {
                            let _ = self.next_char();
                            self.tokens.push(Token {
                                kind: TokenType::EqualEqual,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        } else {
                            self.tokens.push(Token {
                                kind: TokenType::EQUAL,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        }
                    }
                    None => {}
                },
                '~' => match self.peek_char() {
                    Some(next_c) => {
                        if next_c == '=' {
                            let _ = self.next_char();
                            self.tokens.push(Token {
                                kind: TokenType::TildeEqual,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        } else {
                            self.tokens.push(Token {
                                kind: TokenType::TILDE,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        }
                    }
                    None => {}
                },
                '>' => match self.peek_char() {
                    Some(next_c) => {
                        if next_c == '=' {
                            let _ = self.next_char();
                            self.tokens.push(Token {
                                kind: TokenType::GreaterEqual,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        } else {
                            self.tokens.push(Token {
                                kind: TokenType::GREATER,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        }
                    }
                    None => {}
                },
                '<' => match self.peek_char() {
                    Some(next_c) => {
                        if next_c == '=' {
                            let _ = self.next_char();
                            self.tokens.push(Token {
                                kind: TokenType::LessEqual,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        } else {
                            self.tokens.push(Token {
                                kind: TokenType::LESS,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        }
                    }
                    None => {}
                },
                '.' => match self.peek_char() {
                    Some(next_c) => {
                        if next_c == '.' {
                            let _ = self.next_char();
                            self.tokens.push(Token {
                                kind: TokenType::DotLink,
                                lexeme: &self.source[self.start..self.current],
                                literal: Literal::Nil,
                                line: self.line,
                            });
                        } else {
                            let _ = self.lex_number(10);
                        }
                    }
                    None => {
                        panic!("error:unexpect char")
                    }
                },
                '#' => {
                    self.tokens.push(Token {
                        kind: TokenType::POUND,
                        lexeme: &self.source[self.start..self.current],
                        literal: Literal::Nil,
                        line: self.line,
                    });
                }
                ',' => {
                    self.tokens.push(Token {
                        kind: TokenType::COMMA,
                        lexeme: &self.source[self.start..self.current],
                        literal: Literal::Nil,
                        line: self.line,
                    });
                }
                '0'..='9' => {
                    self.current -= 1;
                    match self.lex_number(10) {
                        Ok(_) => {}
                        Err(e) => eprintln!("{}", e),
                    }
                }
                '_' | 'a'..='z' | 'A'..='Z' => {
                    todo!()
                }
                '[' => {
                    if let Some(c) = self.peek_char() {
                        if c == '[' {
                            self.lex_string('[');
                        }
                    }
                }
                '\'' => {
                    self.lex_string('\'');
                }
                '"' => {
                    println!("\":{}:{}", self.start, self.current);
                    let r = self.lex_string('"');
                    print!("{:?}\n", r);
                }
                '\n' => {
                    self.tokens.push(Token {
                        kind: TokenType::LF,
                        lexeme: &self.source[self.start..self.current],
                        literal: Literal::Nil,
                        line: self.line,
                    });
                    self.line += 1;
                }
                _ => eprintln!(
                    "[line{}]Error:\"{}\"is unexpect char",
                    self.line,
                    &self.source[self.start..self.current]
                ),
            },
            None => {}
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn next_char(&mut self) -> Option<char> {
        if !self.is_end() {
            self.current += 1;
            return self.source.chars().nth(self.current - 1);
        }
        None
    }
    fn peek_char(&self) -> Option<char> {
        if !self.is_end() {
            return self.source.chars().nth(self.current);
        }
        None
    }
    fn lex_number(&mut self, radix: u32) -> Result<usize, LuaError> {
        let mut num: f64 = 0f64;
        while let Some(c) = self.next_char() {
            if c.is_digit(radix) {
                num = num * 10f64
                    + c.to_digit(radix).ok_or_else(|| {
                        let l = LuaError::new(
                            self.line,
                            format!("{}", &self.source[self.current - 1..self.current]),
                            "is not a Number".to_string(),
                        );
                        l
                    })? as f64;
            } else if c == '.' {
                {
                    num += self.lex_decimal(radix)?;
                }
                break;
            } else {
                self.current -= 1;
                break;
            }
        }
        self.tokens.push(Token {
            kind: TokenType::NUMBER,
            lexeme: &self.source[self.start..self.current],
            literal: Literal::Number(num),
            line: self.line,
        });
        Ok(0)
    }
    fn lex_decimal(&mut self, radix: u32) -> Result<f64, LuaError> {
        let mut s_num: String = "0.".to_string();
        while let Some(c) = self.next_char() {
            if c.is_digit(radix) {
                s_num.push(c);
            } else {
                self.current -= 1;

                break;
            }
        }

        let num: f64 = s_num.parse().map_err(|_| {
            LuaError::new(
                self.line.clone(),
                format!("{}", s_num),
                "is not decimal".to_string(),
            )
        })?;
        Ok(num)
    }
    fn lex_string(&mut self, quota: char) -> Result<usize, LuaError> {
        let len = (&self.source[self.current..])
            .find(|c| (c == quota) || (c == '\n'))
            .ok_or(LuaError::new(
                self.line,
                quota.to_string(),
                "unfinished string".to_string(),
            ))?;
        if let Some(c) = self.source.chars().nth(self.current + len) {
            if c == '\n' {
                return Err(LuaError::new(
                    self.line,
                    quota.to_string(),
                    "unfinished string".to_string(),
                ));
            } else if c == quota {
                self.current += len + 1;
                println!("{}:{}", self.start, self.current);
                self.tokens.push(Token {
                    kind: TokenType::STRING,
                    lexeme: &self.source[self.start + 1..self.current - 1],
                    literal: Literal::Str(&self.source[self.start + 1..self.current - 1]),
                    line: self.line,
                });
            } else {
                println!("else:{},{},char:{}", self.start, self.current, c);
            }
        }
        println!("None:{},{}", self.start, self.current);
        Ok(len)
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap()
    }
    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap()
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
