#[derive(Debug)]
pub enum ToolError {
    NoneErr(String),
}
impl From<String> for ToolError {
    fn from(value: String) -> Self {
        Self::NoneErr(value)
    }
}
pub fn get_num(str: &String, index: &mut usize) -> Result<usize, ToolError> {
    let len = str.len();
    let str_c = str.chars();
    let mut ca = str_c
        .clone()
        .nth(*index)
        .ok_or(ToolError::NoneErr("Error get char in get num".to_string()))?;
    let mut num = 0usize;
    while ca.is_digit(10) && *index < len {
        num = num * 10
            + ca.clone()
                .to_digit(10)
                .ok_or(ToolError::NoneErr("Error get char in get num".to_string()))?
                as usize;
        *index = *index + 1;
        if *index >= len {
            break;
        }
        ca = str_c
            .clone()
            .nth(*index)
            .ok_or(ToolError::NoneErr("Error get char in get num".to_string()))?;
    }
    Ok(num)
}
