// Here is where we try to REWRITE `slipstream.js` in Rust!
// https://workspace.spirit.com/cvpn/https/ctweb.spirit.com/crewweb/ctwc.aspx
// Such a GROSS url.
// use csv;
// use reqwest;
use scraper::{Html, Selector};
use std::fs;

#[allow(dead_code)] // disable the `dead_code` lint
#[derive(Debug)] // for pretty-printing
struct Flight {
    date: String,
    code: String,
    number: String,
    origin: String,
    destination: String,
    departure_time: String,
    arrival_time: String,
    block_time: String,
    tail_number: String,
    crew: Crew,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Crewmember {
    id: String,
    crew_role: String, // PIC or SIC
    last_name: String,
    first_name: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Crew {
    pic_role: Crewmember,
    sic_role: Crewmember,
}

fn get_pairing_info(document: &Html, search_str: &str) -> Option<String> {
    let selector = Selector::parse("input").expect("Invalid selector");

    document
        .select(&selector)
        .find(|element| element.value().attr("id") == Some(search_str))
        .and_then(|element| element.value().attr("value"))
        .map(String::from)
}

fn get_pairing_grid(document: &Html, search_str: &str) -> Option<String> {
    let selector = Selector::parse("script").expect("Invalid selector");

    document
        .select(&selector)
        .filter_map(|element| element.text().next())
        .find(|&content| content.contains(search_str))
        .map(String::from)
}

fn get_flights(pairing_grid: &String) {
    println!("called `get_flights()`");
}

fn main() {
    // Get raw html from the `crewtrac.html` file (for now)
    let html = fs::read_to_string("tests/crewtrac.html").expect("Unable to read HTML file.");

    // parse the HTML document
    let document = scraper::Html::parse_document(&html);

    // Get the pairing number
    let pairing_number = match get_pairing_info(&document, "PrgNo") {
        Some(content) => content,
        None => "[not found]".to_string(), // or some default value
    };

    // Get the pairing date
    let pairing_date = match get_pairing_info(&document, "PrgDate") {
        Some(content) => content,
        None => "[not found]".to_string(),
    };

    // Get the raw pairing grid data ("gGridText") from inside its <script> tag
    let pairing_grid = match get_pairing_grid(&document, "boob") {
        Some(content) => content,
        None => "no flights found".to_string(),
    };

    // Print pairing number and date
    println!("\nPairing number: {}", &pairing_number);
    println!("Pairing date:   {}", &pairing_date);

    // Print the raw pairing grid
    println!("Pairing grid:   [{}]", &pairing_grid[..16]);

    // Get flights
    let flights = get_flights(&pairing_grid);
    println!("\n{:#?}", &flights);

    // Vectors can only store values of the same type.
    // So the `flights` vector will be a collection of
    // `Flight` structs.
    #[allow(dead_code)]
    //let mut flights: Vec<Flight> = Vec::new();
    // We can loop through the HTML table of flights
    // and add each leg to the `flights` vector.

    // Create some new crewmembers
    let crewmember_1 = Crewmember {
        id: "76148".to_string(),
        crew_role: "PIC".to_string(),
        last_name: "Wendt".to_string(),
        first_name: "Brian".to_string(),
    };
    let crewmember_2 = Crewmember {
        id: String::from("118556"),
        crew_role: String::from("SIC"),
        last_name: String::from("Watkins Jr"),
        first_name: String::from("Timothy"),
    };

    // Pair up a crew for a single flight leg
    let flight_1_crew = Crew {
        pic_role: crewmember_1,
        sic_role: crewmember_2,
    };

    // Create a single flight segment
    #[allow(unused_variables)]
    let flight_1 = Flight {
        date: "08/10/24".to_string(),
        code: "NKS".to_string(),
        number: "1909".to_string(),
        origin: "MCO".to_string(),
        destination: "EWR".to_string(),
        departure_time: "04:58".to_string(),
        arrival_time: "07:20".to_string(),
        block_time: "02:22".to_string(),
        tail_number: "N977NK".to_string(),
        crew: flight_1_crew,
    };

    // {:#?} for pretty printing (vs. {:?})
    //println!("\n{:#?}", flight_1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_pairing_number() {
        let html = fs::read_to_string("tests/crewtrac.html").unwrap();
        let document = scraper::Html::parse_document(&html);
        assert_eq!(get_pairing_info(&document, "PrgNo").is_some(), true);
    }

    #[test]
    fn get_pairing_date() {
        let html = fs::read_to_string("tests/crewtrac.html").unwrap();
        let document = scraper::Html::parse_document(&html);
        assert_eq!(get_pairing_info(&document, "PrgDate").is_some(), true);
    }

    #[test]
    fn get_pairing_gridtext() {
        let html = fs::read_to_string("tests/crewtrac.html").unwrap();
        let document = scraper::Html::parse_document(&html);
        assert_eq!(get_pairing_grid(&document, "gGridText").is_some(), true);
    }
}
