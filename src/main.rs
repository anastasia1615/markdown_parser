use pest::Parser;
use markdown_parser::*;

fn main() -> anyhow::Result<()> {
    let input = "# I love photography";
    let parsed = MarkdownParser::parse(Rule::header1, input)?;
    println!("{:#?}", parsed);
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