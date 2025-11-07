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
}