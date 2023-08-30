use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    TkPlus,
    TkSub,
    TkReserved,
    TkNum,
    TkIllegal,
    TkEof,
}

#[derive(Debug)]
pub enum TokenErr {
    PasIntErr(ParseIntError),
    NoneErr(String),
}
impl From<ParseIntError> for TokenErr {
    fn from(value: ParseIntError) -> Self {
        Self::PasIntErr(value)
    }
}
impl From<String> for TokenErr {
    fn from(value: String) -> Self {
        Self::NoneErr(value)
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub str: &'a str,
}
impl<'a, 'b: 'a> Token<'a> {
    pub fn new(kind: TokenKind, code_s: &'b str) -> Self {
        let ca = code_s.chars().nth(0).unwrap();
        if kind == TokenKind::TkEof {
            return Self {
                kind,
                str: &code_s[0..0],
            };
        }
        if ca.is_digit(10) {
            let len = get_num_len(code_s).unwrap();
            return Self {
                kind: TokenKind::TkNum,
                str: &code_s[0..len],
            };
        } else if ca == '+' || ca == '-' {
            return Self {
                kind: TokenKind::TkReserved,
                str: &code_s[0..1],
            };
        } else {
            return Self {
                kind: TokenKind::TkIllegal,
                str: &code_s[0..1],
            };
        }
    }
    pub fn expect(&self, op: char) -> bool {
        if self.kind != TokenKind::TkReserved || self.str.chars().nth(0).unwrap() != op {
            return false;
        }
        true
    }
    pub fn expect_num(&self) -> Result<usize, TokenErr> {
        if self.kind != TokenKind::TkNum {
            return Err(TokenErr::NoneErr("这不是一个数值".to_string()));
        }
        self.str
            .parse::<usize>()
            .map_err(|e| TokenErr::PasIntErr(e))
    }
    pub fn at_eof(&self) -> bool {
        if self.kind != TokenKind::TkEof {
            return false;
        }
        true
    }
}
fn get_num_2str(str: &str) -> Result<usize, String> {
    let len = str.len();
    let mut index = 0usize;
    let str_c = str.chars();
    let mut ca = str_c
        .clone()
        .nth(index)
        .ok_or("Error get char in get num".to_string())?;
    let mut num = 0usize;
    while ca.is_digit(10) && index < len {
        num = num
            + ca.clone()
                .to_digit(10)
                .ok_or("Error get char in get num".to_string())? as usize;
        index = index + 1;
        if index >= len {
            break;
        }
        ca = str_c
            .clone()
            .nth(index)
            .ok_or("Error get char in get num".to_string())?;
    }
    Ok(num)
}
fn get_num_len(str: &str) -> Result<usize, TokenErr> {
    let len = str.len();
    let mut index = 0usize;
    let str_c = str.chars();
    let mut ca = str_c
        .clone()
        .nth(index)
        .ok_or("Error get char in get num".to_string())?;
    while ca.is_digit(10) && index < len {
        index = index + 1;
        if index >= len {
            break;
        }
        ca = str_c
            .clone()
            .nth(index)
            .ok_or("Error get char in get num".to_string())?;
    }
    Ok(index)
}
