use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MarkdownParser;
#[derive(Error, Debug)]
pub enum MarkdownError {
    #[error("read: failed: {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("parse: failed: {0}")]
    ParseError(#[from] Box<pest::error::Error<Rule>>),
    #[error("input: failed: {0}")]
    InvalidInput(String),
}