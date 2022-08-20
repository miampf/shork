pub mod report;

use std::{fmt::Display, error::Error};

/// All errors go back to this struct
#[derive(Clone, Debug)]
pub struct ShorkError{
    pub e_type: ErrorType,
    pub line: usize,
    pub line_content: String,
    pub pos_in_line: usize,
    pub message: String
}

/// Different error types that can be thrown with a ShorkError
#[derive(Copy, Clone, Debug)]
pub enum ErrorType{
    InterpreterError,
    LexicalError,
    ReadingError,
    SyntaxError,
    TypeError,
}

impl ShorkError{
    /// generate an error from a message and a given position in a source string
    pub fn generate_error(e_type: ErrorType, position: usize, source: String, message: String) -> Self{
        // get the line number of the error
        let mut line = 0;
        let mut pos_in_line = 0;
        for i in 0..position{
            pos_in_line += 1;
            if source.chars().nth(i) == Some('\n'){
                line += 1;
                pos_in_line = 0;
            }
        }

        let lines: Vec<&str> = source.lines().collect();
        let line_content_untrimmed = lines[line];
        let line_content = line_content_untrimmed.trim().to_string();
        
        pos_in_line -= line_content_untrimmed.len()-line_content.len();

        // get the line content there
        Self{
            e_type,
            line,
            line_content,
            pos_in_line,
            message
        }
    }
}

impl Error for ShorkError{}

impl Display for ShorkError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // pinpoint an arrow to the position in line
        let mut arrow = "".to_string();
        for _ in 0..self.pos_in_line {
            arrow += " "
        }

        let line_len = self.line.to_string().len();
        for _ in 1..line_len {
            arrow += " "
        }

        arrow += "^----- Here";

        write!(f, r#"
{} at line {}:
    {} | {}
       {}
{}
        "#, self.e_type, self.line, self.line, self.line_content, arrow, self.message)
    }
}

impl Display for ErrorType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_print = match *self {
            Self::InterpreterError => "Interpreter Error",
            Self::LexicalError => "Lexical Error",
            Self::ReadingError => "Reading Error",
            Self::SyntaxError => "Syntax Error",
            Self::TypeError => "Type Error"
        };
        write!(f, "{}", to_print)
    }
}