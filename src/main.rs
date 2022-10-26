use reqwest::*;
use scraper::*;

fn main() {
    get_id("test");
}

fn get_id(name: &'static str) -> Result<&str> {
    let url = "http://www.ratingupdate.info/?name=".to_owned() + name;
    let namelist = reqwest::blocking::get(url)?.text();

    println!("{:#?}", namelist);
    Ok("ok")
}
