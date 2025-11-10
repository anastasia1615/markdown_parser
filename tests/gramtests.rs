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
    fn ord_list_integration() -> anyhow::Result<()> {
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
}
