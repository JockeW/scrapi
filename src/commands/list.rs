use crate::utils::get_all_scrape_names;

pub fn list() {
    let scrape_names = get_all_scrape_names();

    if scrape_names.is_empty() {
        println!("There are no saved scrapes");
    } else {
        for name in scrape_names {
            println!("{}", name);
        }
    }
}
