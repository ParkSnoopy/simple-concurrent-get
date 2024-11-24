use scraper::{ Html, Selector };



pub fn make_url_lownull<S: AsRef<str>>(route: S) -> String {
    format!("https://evemaps.dotlan.net/route/3:{}",
        route.as_ref(),
    )
}

pub fn parse_text_into_length<S: AsRef<str>>(text: S) -> u64 {
    let distance: u64 = Html::parse_document(text.as_ref())
        .select(&Selector::parse(r#"div[id="navtools"]"#).unwrap())
        .next()
        .expect("Unexpected response format")
        .select(&Selector::parse(r#"table[class="tablelist table-tooltip"]"#).unwrap())
        .next()
        .expect("System Name Invalid")
        .select(&Selector::parse(r#"tr"#).unwrap())
        .last()
        .unwrap()
        .select(&Selector::parse(r#"td"#).unwrap())
        .next()
        .unwrap()
        .inner_html()
        .replace('.', "")
        .trim()
        .parse()
        .expect("Failed to parse route length");

    distance - 1
}
