pub fn get_saved_scrape(name: &str) -> Option<&str> {
    let file_content = include_str!("../scrapes.txt");

    for line in file_content.trim().split('\n') {
        let scrape_name = if line.split(';').collect::<Vec<&str>>()[0] == "combined" {
            line.split(';').collect::<Vec<&str>>()[1]
        } else {
            line.split(';').collect::<Vec<&str>>()[0]
        };

        if scrape_name == name {
            return Some(line);
        }
    }

    None
}