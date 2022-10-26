use reqwest::*;

fn main() {
    get_id("test");
}

fn get_id(name: &'static str) -> Result<&str> {
    let url = "ratingupdate.info/?name={}".to_owned() + name;
    let namelist = reqwest::blocking::get(url);

    println!("{:#?}", namelist);
    Ok("ok")
}
