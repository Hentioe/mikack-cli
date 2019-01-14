use scraper::{Html, Selector};

use crate::errors::*;

pub fn parse_document(html: &str) -> Html {
    Html::parse_document(html)
}

pub fn parse_fragment(html: &str) -> Html {
    Html::parse_fragment(html)
}

pub fn parse_select(selectors: &str) -> Result<Selector> {
    Ok(Selector::parse(selectors)
        .map_err(|_e| err_msg(format!("Selectors: `{}` parsing failed", selectors)))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let html = r#"
            <!DOCTYPE html>
            <meta charset="utf-8">
            <title>Hello, world!</title>
            <h1 class="foo">Hello, <i>world!</i></h1>
        "#;
        let document = parse_document(html);
        let foo_i = document
            .select(&parse_select("h1.foo > i").unwrap())
            .next()
            .unwrap();
        assert_eq!("world!", foo_i.inner_html());

        let meta = document
            .select(&parse_select("meta").unwrap())
            .next()
            .unwrap();
        assert_eq!("utf-8", meta.value().attr("charset").unwrap());

        let ul_dom = r#"
            <ul>
                <li>Foo</li>
                <li>Bar</li>
                <li>Baz</li>
            </ul>
        "#;
        for element in parse_fragment(ul_dom).select(&parse_select("li").unwrap()) {
            assert_eq!("li", element.value().name());
        }
    }
}
