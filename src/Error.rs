#[derive(Debug)]
pub struct LuaError {
    line: usize,
    err_str: String,
    message: String,
}
impl LuaError {
    pub fn new(line: usize, err_str: String, message: String) -> Self {
        Self {
            line,
            err_str,
            message,
        }
    }
}
impl std::fmt::Display for LuaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line{}]Error:\"{}\"{}",
            self.line, self.err_str, self.message
        )
    }
}
