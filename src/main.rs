use markdown_parser::*;
use pest::Parser;

fn main() -> anyhow::Result<()> {
    let input1 = "# It's a funny thing, when you talk to God, you're religious, but when he talks to you, you're a psychopath.";
    let parsed1 = MarkdownParser::parse(Rule::header1, input1)?;
    println!("{:#?}", parsed1);
    let input2 = "## Instead of slashing my wrists, I just write a bunch of really crummy songs.";
    let parsed2 = MarkdownParser::parse(Rule::header2, input2)?;
    println!("{:#?}", parsed2);
    let input3 = "### We're the only species who hunts for sport.";
    let parsed3 = MarkdownParser::parse(Rule::header3, input3)?;
    println!("{:#?}", parsed3);
    let input4 = "**Bloody Kisses**";
    let parsed4 = MarkdownParser::parse(Rule::bold, input4)?;
    println!("{:#?}", parsed4);
    let input5 = "*Oye como va mi ritmo, bueno pa gosar mulata*";
    let parsed5 = MarkdownParser::parse(Rule::italic, input5)?;
    println!("{:#?}", parsed5);
    let input5 = "*Oye como va mi ritmo, bueno pa gosar mulata*";
    let parsed5 = MarkdownParser::parse(Rule::italic, input5)?;
    println!("{:#?}", parsed5);
    let input6 = "> I'm a big fan of the effects of alcohol.";
    let parsed6 = MarkdownParser::parse(Rule::quote, input6)?;
    println!("{:#?}", parsed6);
    let input7 = "*** August 17, 1993 ***";
    let parsed7 = MarkdownParser::parse(Rule::boit, input7)?;
    println!("{:#?}", parsed7);
    let input8 = r#"- Anesthesia
- Christian Woman
- Love You to Death
- Kill You Tonight
- The Profit of Doom
- September Sun"#;
    let parsed8 = MarkdownParser::parse(Rule::unord_list, input8)?;
    println!("{:#?}", parsed8);
    let input9 = r#"1. Christian Woman
2. Bloody Kisses
3. Too Late: Frozen
4. Blood & Fire
5. Can't Lose You
6. Summer Breeze
7. Set Me on Fire
8. Suspended in Dusk
9. Black No.1"#;
    let parsed9 = MarkdownParser::parse(Rule::ord_list, input9)?;
    println!("{:#?}", parsed9);
    let input10 = "`let mut x = 10;`";
    let parsed10 = MarkdownParser::parse(Rule::code, input10)?;
    println!("{:#?}", parsed10);
    let input11 = "[i dare you to listen to it if you actually read it](https://open.spotify.com/track/5mSEGxrMVHBXkyU7aThjrw?si=40516d7417bf4f79)";
    let parsed11 = MarkdownParser::parse(Rule::link, input11)?;
    println!("{:#?}", parsed11);
    let input12 = "![i believe in god](https://64.media.tumblr.com/d2f312b1cb21158ae583d1f95d5e4bc2/6ce41997e6fa4c16-f7/s1280x1920/8f8990223e4b3abb2b214e8c61db3baeb9530642.jpg)";
    let parsed12 = MarkdownParser::parse(Rule::img, input12)?;
    println!("{:#?}", parsed12);
    let input13 = "***";
    let parsed13 = MarkdownParser::parse(Rule::hor_line, input13)?;
    println!("{:#?}", parsed13);
    let input14 = "\n";
    let parsed14 = MarkdownParser::parse(Rule::new, input14)?;
    println!("{:#?}", parsed14);
    let input15 = "She is in love with herself, She likes the dark";
    let parsed15 = MarkdownParser::parse(Rule::paragraph, input15)?;
    println!("{:#?}", parsed15);
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
fn link() {
    let input = "[one of the best tracks of 2025 imho](https://open.spotify.com/track/2uXxooVssBJB8Llk2dB5kf?si=1fcdb8b947eb4334)";
    let result = MarkdownParser::parse(Rule::link, input);
    assert!(result.is_ok());
    let pair = result.unwrap().next().unwrap();
    assert_eq!(pair.as_str(), input);
}
#[test]
fn img() {
    let input = "![also me](https://i.pinimg.com/474x/87/de/91/87de91d21ac023ac11c553eab9ceb0f4.jpg)";
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
