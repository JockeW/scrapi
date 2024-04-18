use crate::utils::get_all_scrape_names;

pub fn list() {
    let scrape_names = get_all_scrape_names();

    for name in scrape_names {
        println!("{}", name);
    }
}
