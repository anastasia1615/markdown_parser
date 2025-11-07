use pest::Parser;
use markdown_parser::*;

fn main() -> anyhow::Result<()> {
    let input1 = "# I love photography";
    let parsed1 = MarkdownParser::parse(Rule::header1, input1)?;
    println!("{:#?}", parsed1);
    let input2 = "## idk what to write";
    let parsed2 = MarkdownParser::parse(Rule::header2, input2)?;
    println!("{:#?}", parsed2);
    let input3 = "### i ran out of ideas sorry";
    let parsed3 = MarkdownParser::parse(Rule::header3, input3)?;
    println!("{:#?}", parsed3);
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