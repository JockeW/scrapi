use scraper::{ElementRef, Html, Selector};
use thirtyfour::{DesiredCapabilities, WebDriver, WebDriverCommands};

#[tokio::main]
pub async fn inspect(url: String, filter: Option<String>, search: Option<String>) {
    // let html = reqwest::blocking::get(&url).unwrap().text().unwrap();
    // let document = Html::parse_document(&html);

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).await.unwrap();

    // Navigate to https://wikipedia.org.
    driver.get(url).await.unwrap();

    let test = driver.find_element(thirtyfour::By::Name("html")).await.unwrap();
    println!("TESTING: {}", test.inner_html().await.unwrap());
    // if let Some(filter) = filter {
    //     let selector = Selector::parse(&filter).expect("Not a valid selector");
    //     let element_ref: Vec<ElementRef> = document.select(&selector).collect();

    //     for element in element_ref {
    //         let test = element.html();
    //         println!("{}", test);
    //     }
    // }

    // println!("{}", document.html());
    // if let Some(search) = search {
    //     document.html().
    // }

    driver.quit().await.unwrap();
}
