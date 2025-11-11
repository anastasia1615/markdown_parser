use anyhow::anyhow;
use markdown_parser::*;
use pest::Parser;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn int_header1() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::header1, "# Hail To The Thief")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "# Hail To The Thief");
        assert!(MarkdownParser::parse(Rule::header1, "# ").is_err());
        assert!(MarkdownParser::parse(Rule::header1, "Amnesiac").is_err());
        Ok(())
    }
    #[test]
    fn int_header2() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::header2, "## The Bends")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "## The Bends");
        assert!(MarkdownParser::parse(Rule::header2, "## ").is_err());
        assert!(MarkdownParser::parse(Rule::header2, "# Kid A").is_err());
        Ok(())
    }
    #[test]
    fn int_header3() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::header3, "### OK Computer")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "### OK Computer");
        assert!(MarkdownParser::parse(Rule::header3, "### ").is_err());
        assert!(MarkdownParser::parse(Rule::header3, "## Pablo Honey").is_err());
        Ok(())
    }
    #[test]
    fn int_bold() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::bold, "**Anyone Can Play Guitar**")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "**Anyone Can Play Guitar**");
        assert!(MarkdownParser::parse(Rule::bold, "**").is_err());
        assert!(MarkdownParser::parse(Rule::bold, "Vegetable").is_err());
        Ok(())
    }
    #[test]
    fn int_italic() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::italic, "*Weird Fishes (Arpeggi)*")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "*Weird Fishes (Arpeggi)*");
        assert!(MarkdownParser::parse(Rule::italic, "*").is_err());
        assert!(MarkdownParser::parse(Rule::italic, "Paranoid Android").is_err());
        Ok(())
    }
    #[test]
    fn int_quote() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::quote, "> I'm not afraid of computers taking over the world. They're just sitting there. I can hit them with a two by four.")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "> I'm not afraid of computers taking over the world. They're just sitting there. I can hit them with a two by four.");
        assert!(MarkdownParser::parse(Rule::quote, ">").is_err());
        assert!(MarkdownParser::parse(Rule::quote, "So I don't cry anymore, I just beat people up. It's a lot more fun.").is_err());
        Ok(())
    }
    #[test]
    fn int_boit() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::boit, "*** Band of agony and despair ***")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "*** Band of agony and despair ***");
        assert!(MarkdownParser::parse(Rule::boit, "**").is_err());
        assert!(MarkdownParser::parse(Rule::boit, "* text").is_err());
        Ok(())
    }
    #[test]
    fn int_unord() -> anyhow::Result<()> {
        let input = r#"- MK 1
- Down Is the New Up
- Go Slowly
- MK 2
- Last Flowers
- Up on the Ladder
- Bangers + Mash
- 4 Minute Warning"#;
        let mut got = MarkdownParser::parse(Rule::unord_list, input)?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(
            got.as_str(),
        "- MK 1
- Down Is the New Up
- Go Slowly
- MK 2
- Last Flowers
- Up on the Ladder
- Bangers + Mash
- 4 Minute Warning");
        assert!(MarkdownParser::parse(Rule::unord_list, "1.").is_err());
        assert!(MarkdownParser::parse(Rule::unord_list, "MK 1").is_err());
        Ok(())
    }
    #[test]
    fn int_ord() -> anyhow::Result<()> {
    let input = r#"1. 2 + 2 = 5
2. Sit Down. Stand Up.
3. Sail to the Moon.
4. Backdrifts.
5. Go to Sleep.
6. Where I End and You Begin.
7. We Suck Young Blood.
8. The Gloaming.
9. There There.
10. I Will.
11. A Punchup at a Wedding.
12. Myxomatosis.
13. Scatterbrain.
14. A Wolf at the Door."#;
    let mut got = MarkdownParser::parse(Rule::ord_list, input)?;
    let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
    assert_eq!(got.as_str(), "1. 2 + 2 = 5
2. Sit Down. Stand Up.
3. Sail to the Moon.
4. Backdrifts.
5. Go to Sleep.
6. Where I End and You Begin.
7. We Suck Young Blood.
8. The Gloaming.
9. There There.
10. I Will.
11. A Punchup at a Wedding.
12. Myxomatosis.
13. Scatterbrain.
14. A Wolf at the Door.");
    assert!(MarkdownParser::parse(Rule::ord_list, "1.").is_err());
    assert!(MarkdownParser::parse(Rule::ord_list, "Hello").is_err());
    Ok(())
    }
    #[test]
    fn int_code() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::code,"`println!(\"hello world!\");`",)?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "`println!(\"hello world!\");`");
        assert!(MarkdownParser::parse(Rule::code, "`").is_err());
        assert!(MarkdownParser::parse(Rule::code, "println!(\"goodbye world!\");").is_err());
        Ok(())
    }
    #[test]
    fn int_link() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::link, "[i fweaking love radiohead](https://open.spotify.com/track/3oOHf32BT7dkzI4tAfNZun?si=f0a4de3cbe7a41bb)")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "[i fweaking love radiohead](https://open.spotify.com/track/3oOHf32BT7dkzI4tAfNZun?si=f0a4de3cbe7a41bb)");
        assert!(MarkdownParser::parse(Rule::link, "[description]").is_err());
        assert!(MarkdownParser::parse(Rule::link, "(just link)").is_err());
        Ok(())
    }
    #[test]
    fn int_img() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::img, "![literally me](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQc2SzjhiHSE9z3MyvTY4wPNeup5zDJw4LLyA&s)")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "![literally me](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQc2SzjhiHSE9z3MyvTY4wPNeup5zDJw4LLyA&s)");
        assert!(MarkdownParser::parse(Rule::img, "![description]").is_err());
        assert!(MarkdownParser::parse(Rule::img, "(just link)").is_err());
        Ok(())
    }
    #[test]
    fn int_horisontal() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::hor_line, "---")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "---");
        assert!(MarkdownParser::parse(Rule::hor_line, "--").is_err());
        assert!(MarkdownParser::parse(Rule::hor_line, "__").is_err());
        Ok(())
    }
    #[test]
    fn int_new() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::new, "\n")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "\n");
        assert!(MarkdownParser::parse(Rule::new, " ").is_err());
        assert!(MarkdownParser::parse(Rule::new, "letters").is_err());
        Ok(())
    }
    #[test]
    fn int_par() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::paragraph, "The mongril cat came home holding half a head")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "The mongril cat came home holding half a head");
        assert!(MarkdownParser::parse(Rule::paragraph, "\n").is_err());
        assert!(MarkdownParser::parse(Rule::paragraph, "").is_err());
        Ok(())
    }
}
