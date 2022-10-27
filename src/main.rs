use reqwest::*;
use scraper::*;

#[derive(Debug)]
struct Search {
    name: String,
    id: String,
    character: String
}

fn main() {
    get_id("Nemi");
}

fn get_id(target_name: &'static str) -> Result<Vec<Search>> {
    let url = "http://www.ratingupdate.info/?name=".to_owned() + target_name;
    let body = reqwest::blocking::get(url)?.text()?;
    let document = Html::parse_document(&body);
    let mut ret_value: Vec<Search> = Vec::new();

    let selector = Selector::parse(r#"a[class="maybe_long_name"]"#).unwrap();
    for entry in document.select(&selector) {
        let name = entry.value().attr("title").ok_or("");
        let url = entry.value().attr("href").ok_or("");
        if name.unwrap() == target_name {
            ret_value.push(Search{
                name: name.unwrap().to_string(),
                id: url.unwrap().replace("/player/", "")[..15].to_string(),
                character: url.unwrap()[24..].to_string()
            });
        }
    }
    println!("{:?}", ret_value);
    Ok(ret_value)
}
