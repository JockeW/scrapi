use scraper::{Html, Selector, ElementRef};

pub fn inspect(url: String, filter: Option<String>, search: Option<String>) {
    let html = reqwest::blocking::get(&url).unwrap().text().unwrap();
    let document = Html::parse_document(&html);

    if let Some(filter) = filter {
        let selector = Selector::parse(&filter).expect("Not a valid selector");
        let element_ref: Vec<ElementRef> = document.select(&selector).collect();

        for element in element_ref {
            let test = element.html();
            println!("{}", test);
        }
    }

    println!("{}", document.html());
    if let Some(search) = search {
        document.html().
    }
}