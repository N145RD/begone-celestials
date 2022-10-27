use reqwest::*;
use scraper::*;

fn main() {
    get_id("test");
}

fn get_id(target_name: &'static str) -> Result<&str> {
    let url = "http://www.ratingupdate.info/?name=".to_owned() + target_name;
    let body = reqwest::blocking::get(url)?.text()?;
    let document = Html::parse_document(&body);

    let selector = Selector::parse(r#"a[class="maybe_long_name"]"#).unwrap();
    for entry in document.select(&selector) {
        let name = entry.value().attr("title").ok_or("");
        let url = entry.value().attr("href").ok_or("");
        if name.unwrap() == target_name {
            println!("Found matching name : {} at link {}", name.unwrap(), url.unwrap());
        }
    }
    Ok("ok")
}
