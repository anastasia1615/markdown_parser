use pest_derive::Parser;
use thiserror::Error;
/// markdown parser struct is generated from `grammar.pest`.
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MarkdownParser;
/// error handling
#[derive(Error, Debug)]
pub enum MarkdownError {
    /// file read error
    #[error("read: failed: {0}")]
    FileReadError(#[from] std::io::Error),
    /// parse error
    #[error("parse: failed: {0}")]
    ParseError(#[from] Box<pest::error::Error<Rule>>),
    /// invalid input
    #[error("input: failed: {0}")]
    InvalidInput(String),
}
/// grammar rules doc
#[allow(dead_code)]
pub enum MarkdownGrammar {
    /// Document contains the rules below
    Doc,
    /// header l1: `# Header`.
    Header1,
    /// header l2: `## Header`.
    Header2,
    /// haeder l3: `### Header`.
    Header3,
    /// bold text: `**bold**`.
    Bold,
    /// italic text: `*italic*`.
    Italic,
    /// bold +italic: `***bold+italic***`.
    BoIt,
    /// blockquote: `> quote`.
    Quote,
    /// unordered list: `- elem` / `* elem` / `+ elem`.
    UnordList,
    /// ordered list: `1. elem` `2. elem`.
    OrdList,
    /// code: `` `code` ``.
    Code,
    /// code block:
    /// ```rust
    /// fn main() {}
    /// ```
    CodeBl,
    /// link: `[text](url)`.
    Link,
    /// image: `![alt](url)`.
    Img,
    /// horzontal line: `---` / `***` / `___`.
    HorLine,
    /// newline.
    New,
    /// paragraph.
    Paragraph,
}
