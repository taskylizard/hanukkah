use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_jsonlines::json_lines;
use std::io::Result;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Customer {
    pub customerid: i64,
    pub name: String,
    pub address: String,
    pub citystatezip: String,
    pub birthdate: String,
    pub phone: String,
    pub timezone: String,
    pub lat: f64,
    pub long: f64,
}

#[derive(Debug)]
pub struct ParsedCustomer {
    pub customerid: i64,
    pub name: String,
    pub address: String,
    pub citystatezip: String,
    pub birthdate: NaiveDate,
    pub phone: String,
    pub timezone: String,
    pub lat: f64,
    pub long: f64,
}

fn main() -> Result<()> {
    let years = [1915, 1927, 1939, 1951, 1963, 1975, 1987, 1999, 2011];
    let city = "Jamaica";

    let customers = json_lines("data/noahs-customers.jsonl")?.collect::<Result<Vec<Customer>>>()?;

    let customer = customers
        .into_iter()
        .filter(|c| c.citystatezip.contains(city))
        // Use Customer struct but change the birthdate field to a chrono::NaiveDate
        .map(|c| ParsedCustomer {
            customerid: c.customerid,
            name: c.name,
            address: c.address,
            citystatezip: c.citystatezip,
            // Parse the birthdate with chrono
            birthdate: NaiveDate::parse_from_str(&c.birthdate, "%Y-%m-%d").unwrap(),
            phone: c.phone,
            timezone: c.timezone,
            lat: c.lat,
            long: c.long,
        })
        .filter(|c| years.contains(&c.birthdate.year()))
        .filter(|c| c.birthdate.month() == 5 || c.birthdate.month() == 6)
        .filter(|c| (c.birthdate.month() == 5 && c.birthdate.day() >= 21) || (c.birthdate.month() == 6 && c.birthdate.day() <= 22))
        .collect::<Vec<ParsedCustomer>>();

    println!("{:?}", customer[0].phone);
    Ok(())
}
