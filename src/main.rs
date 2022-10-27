use reqwest::*;
use scraper::*;

#[derive(Debug)]
struct Entry {
    name: String,
    id: String,
    character: String,
    full_url: String
}

struct Match {
    floor: i32,
    //opponent: Entry,
    result: bool
}

fn main() {
    let entries: Vec<Entry> = get_id("Nemi").unwrap();

    for entry in entries {
        let eligible = validate_entry(&entry).unwrap();
        println!("Entry of {} with id {} playing {} at {} is {}", entry.name, entry.id, entry.character, entry.full_url, eligible);
    }
}

fn row_to_match(raw: String) -> Result<Match> {
    let fragment = Html::parse_fragment(&raw);
    let row_selector = Selector::parse("td").unwrap();
    let mut array = fragment.select(&row_selector);
    let mut return_value = Match{floor: 0, result: true};

    array.next();
    array.next();
    let floor = array.next();
    if floor.unwrap().inner_html() != "C" {
        return_value.floor = floor.unwrap().inner_html().parse::<i32>().unwrap();
    } else {
        return_value.floor = 10;
    }

    array.next();
    array.next();
    array.next();
    array.next();
    let result = array.next();
    let inside_fragment = Html::parse_fragment(&result.unwrap().inner_html());
    let inside_selector = Selector::parse("span").unwrap();
    let mut element = inside_fragment.select(&inside_selector);
    if element.next().expect("").value().attr("title").unwrap() != "0.0%" {
        return_value.result = true;
    } else {
        return_value.result = false;
    }

    Ok(return_value)
}

fn validate_entry(entry: &Entry) -> Result<bool> {
    let query = "http://ratingupdate.info".to_owned() + &entry.full_url;
    let body = reqwest::blocking::get(query)?.text()?;
    let document = Html::parse_document(&body);
    let row_selector = Selector::parse("tr").unwrap();
    let selector = Selector::parse(r#"div[id="history"]"#).unwrap();

    let row = document.select(&selector).next().unwrap();
    for row in document.select(&row_selector) {
        let one = row_to_match(row.inner_html()).unwrap();
        if one.floor == 10 && one.result == true {
            return Ok(true);
        }
    }
    Ok(false)
}

fn get_id(target_name: &'static str) -> Result<Vec<Entry>> {
    let search_url = "http://www.ratingupdate.info/?name=".to_owned() + target_name;
    let body = reqwest::blocking::get(search_url)?.text()?;
    let document = Html::parse_document(&body);
    let mut ret_value: Vec<Entry> = Vec::new();

    let selector = Selector::parse(r#"a[class="maybe_long_name"]"#).unwrap();
    for entry in document.select(&selector) {
        let name = entry.value().attr("title").ok_or("");
        let url = entry.value().attr("href").ok_or("");
        if name.unwrap() == target_name {
            ret_value.push(Entry{
                name: name.unwrap().to_string(),
                id: url.unwrap().replace("/player/", "")[..15].to_string(),
                character: url.unwrap()[24..].to_string(),
                full_url: url.unwrap().to_string()
            });
        }
    }
    Ok(ret_value)
}
