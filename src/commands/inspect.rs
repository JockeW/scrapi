use scraper::{Html, Selector, ElementRef};

pub fn inspect(url: String, filter: Option<String>, search: Option<String>) {
    // let html = reqwest::blocking::get(&url).unwrap().text().unwrap();
    // let document = Html::parse_document(&html);

    // for (index, s) in filter.iter().enumerate() {
    //     let selector = Selector::parse(&s).expect("Not a valid selector");
    //     let element_ref: Vec<ElementRef> = document.select(&selector).collect();
    // }
    
}
