use scraper::{ElementRef, Html, Selector};

use crate::errors::*;

pub fn parse_document(html: &str) -> Html {
    Html::parse_document(html)
}

pub fn parse_fragment(html: &str) -> Html {
    Html::parse_fragment(html)
}

pub fn parse_select(selectors: &str) -> Result<Selector> {
    Ok(Selector::parse(selectors)
        .map_err(|_e| err_msg(format!("selectors: `{}` parsing failed", selectors)))?)
}

pub fn find_text<'a>(doc: &'a Html, selectors: &str) -> Result<&'a str> {
    doc.select(&parse_select(selectors)?)
        .next()
        .ok_or(err_msg(format!(
            "no element found based on selector ‘{}’",
            selectors
        )))?
        .text()
        .next()
        .ok_or(err_msg(format!(
            "no text value found based on selector ‘{}’",
            selectors
        )))
}

pub fn find_text_in_element<'a>(elem: &'a ElementRef, selectors: &str) -> Result<&'a str> {
    elem.select(&parse_select(selectors)?)
        .next()
        .ok_or(err_msg(format!(
            "no element found based on selector ‘{}’",
            selectors
        )))?
        .text()
        .next()
        .ok_or(err_msg(format!(
            "no text value found based on selector ‘{}’",
            selectors
        )))
}

pub fn find_attr<'a>(doc: &'a Html, selectors: &str, attr: &str) -> Result<&'a str> {
    doc.select(&parse_select(selectors)?)
        .next()
        .ok_or(err_msg(format!(
            "no element found based on selector ‘{}’",
            selectors
        )))?
        .value()
        .attr(attr)
        .ok_or(err_msg(format!(
            "no attr '{}' found based on selector ‘{}’",
            attr, selectors
        )))
}

pub fn find_list_attr<'a>(doc: &'a Html, selectors: &str, attr: &str) -> Result<Vec<&'a str>> {
    let mut attr_list: Vec<&str> = vec![];
    for elem in doc.select(&parse_select(selectors)?) {
        let attr = elem.value().attr(attr).ok_or(err_msg(format!(
            "no '{}' found based on selector ‘{}’",
            attr, selectors
        )))?;
        attr_list.push(attr);
    }
    Ok(attr_list)
}

pub fn count(doc: &Html, selectors: &str) -> Result<usize> {
    Ok(doc.select(&parse_select(selectors)?).count())
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
