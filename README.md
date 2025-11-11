## markdown Parser by zrznklbbk
парсер буде обробляти синтаксис Markdown, а саме заголовки трьох рівнів, жирний шрифт, курсив, жирний+курсив, нумеровані і ненумеровані списки, цитати, код, лінки, зображення та горизонтальні лінії. файл .md буде парситись, передаючись до структури MarkdownParser. до тексту у файлі будуть застосовуватись правила граматики. результат буде виводитись в кнсоль якщо знайдеться елемент синтаксису.

[Source](https://www.markdownguide.org/cheat-sheet/)
![фотка main.rs](image.png)
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