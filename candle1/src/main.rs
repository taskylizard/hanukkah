use serde::{Deserialize, Serialize};
use serde_jsonlines::json_lines;
use std::io::Result;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Customer {
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

fn string_to_phone_number(s: &str) -> Option<String> {
    let mut phone_number = String::new();
    for char in s.to_ascii_uppercase().chars() {
        let n = match char {
            'A'..='C' => '2',
            'D'..='F' => '3',
            'G'..='I' => '4',
            'J'..='L' => '5',
            'M'..='O' => '6',
            'P'..='S' => '7',
            'T'..='V' => '8',
            'W'..='Z' => '9',
            _ => return None,
        };
        phone_number.push(n);
    }
    Some(phone_number)
}

fn main() -> Result<()> {
    let customers = json_lines("data/noahs-customers.jsonl")?.collect::<Result<Vec<Customer>>>()?;

    for customer in customers {
        let last_name = customer.name.split_ascii_whitespace().last().unwrap();
        if let Some(phone_number) = string_to_phone_number(last_name) {
            if phone_number == customer.phone.replace('-', "") {
                println!(
                    "Phone number: {}, Phone: {}, Name: {}",
                    phone_number, customer.phone, customer.name
                );
            }
        }
    }
    Ok(())
}
