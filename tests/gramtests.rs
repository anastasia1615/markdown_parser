use pest::Parser;
use anyhow::anyhow;
use markdown_parser::*;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn int_header1() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::header1, "# I love beer")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "# I love beer");
        assert!(MarkdownParser::parse(Rule::header1, "# ").is_err());
        assert!(MarkdownParser::parse(Rule::header1, "I").is_err());
        Ok(())
    }
    #[test]
    fn int_header2() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::header2, "## I don't know what to write")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "## I don't know what to write");
        assert!(MarkdownParser::parse(Rule::header2, "## ").is_err());
        assert!(MarkdownParser::parse(Rule::header2, "# Rawr").is_err());
        Ok(())
    }
    #[test]
    fn int_header3() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::header3, "### Lalalalala")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "### Lalalalala");
        assert!(MarkdownParser::parse(Rule::header3, "### ").is_err());
        assert!(MarkdownParser::parse(Rule::header3, "## Yo").is_err());
        Ok(())
    }
    #[test]
    fn int_bold() -> anyhow::Result<()>{
        let mut got = MarkdownParser::parse(Rule::bold, "**a big bold beautiful journey**")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "**a big bold beautiful journey**");
        assert!(MarkdownParser::parse(Rule::bold, "**").is_err());
        assert!(MarkdownParser::parse(Rule::bold, "bold").is_err());
        Ok(())
    }
    #[test]
    fn int_italic() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::italic, "*sono fuori di testa*")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "*sono fuori di testa*");
        assert!(MarkdownParser::parse(Rule::italic, "*").is_err());
        assert!(MarkdownParser::parse(Rule::italic, "ciao").is_err());
        Ok(())
    }
    #[test]
    fn int_quote() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::quote, "> So I don't cry anymore, I just beat people up. It's a lot more fun.")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "> So I don't cry anymore, I just beat people up. It's a lot more fun.");
        assert!(MarkdownParser::parse(Rule::quote, ">").is_err());
        assert!(MarkdownParser::parse(Rule::quote, "in her place one hundred candles burning").is_err());
        Ok(())
    }
}