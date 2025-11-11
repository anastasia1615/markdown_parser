## markdown Parser by zrznklbbk
парсер буде обробляти синтаксис Markdown, а саме заголовки трьох рівнів, жирний шрифт, курсив, жирний+курсив, нумеровані і ненумеровані списки, цитати, код, лінки, зображення та горизонтальні лінії. файл .md буде парситись, передаючись до структури MarkdownParser. до тексту у файлі будуть застосовуватись правила граматики. результат буде виводитись в кнсоль якщо знайдеться елемент синтаксису.

[Source](https://www.markdownguide.org/cheat-sheet/)
![фотка main.rs](image.png)
## Grammar & Parsing

The parser uses `pest` grammar rules to recognize Markdown syntax. The table below shows each Markdown element, an example, and the corresponding grammar rule:

| Markdown               | Example                                   | pest                 |
|------------------------|-------------------------------------------|----------------------|
| Header 1               | `# first`                                 | `header1`            |
| Header 2               | `## second`                             | `header2`            |
| Header 3               | `### third`                             | `header3`            |
| Bold                   | `**bldtxt**`                                | `bold`               |
| Italic                 | `*ittxt*`                                  | `italic`             |
| Bold + Italic          | `***boittxt***`                              | `boit`               |
| Blockquote             | `> quote`                                 | `quote`              |
| Unordered List         | `- elem` / `* elem` / `+ elem`           | `unord_list`         |
| Ordered List           | `1. elem`                                 | `ord_list`           |
| Inline Code            | `` `code` ``                              | `code`               |
| Code Block             | ```` ```rust ... ``` ````                  | `codebl`             |
| Link                   | `[text](url)`                             | `link`               |
| Image                  | `![alt](url)`                             | `img`                |
| Horizontal Line        | `---` / `***` / `___`                     | `hor_line`           |
| Paragraph              | `Lalalalala.`                              | `paragraph`          |
| Newline                | `\n`                                      | `new`                |

## code example
```rust
    use pest::Parser;
    use Markdown_parser::*;

    fn main() -> anyhow::Result<()> {
        let input = "# I love photography";
        let parsed = MarkdownParser::parse(Rule::header1, input)?;
        println!("{:#?}", parsed);
        Ok(())
    }
```