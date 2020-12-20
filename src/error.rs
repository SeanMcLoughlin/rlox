use thiserror::Error;

fn format_err(line: &usize, message: String) -> String {
    format!("[line {}] Error: {}", line, message)
}

#[derive(Error, Debug, PartialEq)]
pub enum LoxError {
    #[error("{}", format_err(line, format!("Invalid Token: {}", token)))]
    InvalidToken { line: usize, token: String },

    #[error("{}", format_err(line, "Unterminated string".to_string()))]
    UnterminatedString { line: usize },

    #[error("{}", format_err(line, "Unterminated float".to_string()))]
    UnterminatedFloat { line: usize },
}
