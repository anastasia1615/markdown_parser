use anyhow::Result;
use markdown_parser::*;
use pest::Parser;
use std::fs;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum CliError {
    #[error("file error: {0}")]
    FileError(#[from] std::io::Error),
    #[error("parse error: {0}")]
    ParseError(#[from] Box<pest::error::Error<Rule>>),
    #[error("invalid command: {0}")]
    InvalidCommand(String),
}
fn help() {
    println!("markdown parser");
    println!("how to use:");
    println!("parse <file> - parse a file");
    println!("help - this message again");
    println!("credits - author info");
    println!("to call a command, write");
    println!("cargo run -- command");
    println!("to parse a file, write");
    println!("cargo run -- parse filename.md ");
}
fn credits() {
    println!("markdown parser");
    println!("by Anastasiia Pokormiak, CS-2");
}
fn parse_file(file_path: &str) -> Result<(), CliError> {
    let input_file = fs::read_to_string(file_path)?;
    let parsed = MarkdownParser::parse(Rule::doc, &input_file)
        .map_err(|e| CliError::ParseError(Box::new(e)))?;
    println!("{:#?}", parsed);
    Ok(())
}
fn main() -> Result<(), CliError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        help();
        return Ok(());
    }
    let com = &args[1];
    if com == "help" {
        help();
    } else if com == "credits" {
        credits();
    } else if com == "parse" {
        if args.len() < 3 {
            eprintln!("no file");
            help();
            return Ok(());
        }
        parse_file(&args[2])?;
    } else {
        return Err(CliError::InvalidCommand(com.clone()));
    }
    Ok(())
}
#[test]
fn header1() {
    let input = "# oh please please please let me get what i want";
    let result = MarkdownParser::parse(Rule::header1, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn header2() {
    let input = "## Be quiet and drive (far away)";
    let result = MarkdownParser::parse(Rule::header2, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn header3() {
    let input = "### My mind is a mountain";
    let result = MarkdownParser::parse(Rule::header3, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn bold() {
    let input = "**Digital bath**";
    let result = MarkdownParser::parse(Rule::bold, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn italic() {
    let input = "*My own summer*";
    let result = MarkdownParser::parse(Rule::italic, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn boit() {
    let input = "*** Entombed ***";
    let result = MarkdownParser::parse(Rule::boit, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn quote() {
    let input = "> Xerces";
    let result = MarkdownParser::parse(Rule::quote, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn unord_list() {
    let input = r#"- 7 Words
- Bored
- Around The Fur
- Digital Bath
- Bloody Cape"#;
    let result = MarkdownParser::parse(Rule::unord_list, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn ord_list() {
    let input = r#"1. My Own Summer (Shove It)
2. Lhabia
3. Mascara
4. Around the Fur
5. Rickets
6. Be Quiet and Drive
7. Lotion
8. Dai the Flu
9. Headup
10. MX"#;
    let result = MarkdownParser::parse(Rule::ord_list, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn code() {
    let input = "`let mut value = 15`";
    let result = MarkdownParser::parse(Rule::code, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn codebl() {
    let input = "```rust
fn rosemary() {
    println!(\"There's no sound, but the engine's drone\");
```";
    let result = MarkdownParser::parse(Rule::codebl, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn link() {
    let input = "[i dare you to listen to this song if you actually read it](https://open.spotify.com/track/2uXxooVssBJB8Llk2dB5kf?si=1fcdb8b947eb4334)";
    let result = MarkdownParser::parse(Rule::link, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn img() {
    let input =
        "![also me](https://i.pinimg.com/474x/87/de/91/87de91d21ac023ac11c553eab9ceb0f4.jpg)";
    let result = MarkdownParser::parse(Rule::img, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn horisontal() {
    let input = "___";
    let result = MarkdownParser::parse(Rule::hor_line, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn new() {
    let input = "\n";
    let result = MarkdownParser::parse(Rule::new, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn paragraph() {
    let input = "There's a room, We hang in space, It's clear, cold.";
    let result = MarkdownParser::parse(Rule::paragraph, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn doc() {
    let input = r#"# The description of how I love Deftones
## Deftones is an American alternative metal band
### years active: 1988 - now

Deftones are just like ***Radiohead*** but for those who have *sex* **lol**

> "Old Korn records had so much intensity." – Chino Moreno

- Chino Moreno
- Stephen Carpenter
- Abe Cunningham
- Frank Delgado
- Sergio Vega

1. Change (In the House of Flies)
2. My Own Summer (Shove It)
3. Digital Bath
4. Be Quiet and Drive (Far Away)
5. Diamond Eyes

Listen to their album [White Pony](https://open.spotify.com/album/1DP3I0S7U3jN6wqftoQcQ3?si=8zZ1NbxSSiGz4W9GjaXDTg)

![Deftones live in concert](https://upload.wikimedia.org/wikipedia/commons/0/0a/Deftones_live_2013.jpg)

---

`println!("I adore my music taste");`

```rust
fn main() {
    let band = "Deftones";
    println!("I love {}", band);
}
"#;
    let result = MarkdownParser::parse(Rule::doc, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input)
}
