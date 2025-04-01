use helper::prelude::*;
use std::collections::HashMap;

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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Order {
    pub orderid: i64,
    pub customerid: i64,
    pub ordered: String,
    pub shipped: String,
    pub items: Vec<Item>,
    pub total: f64,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Item {
    pub sku: String,
    pub qty: i64,
    pub unit_price: f64,
}

fn main() -> Result<()> {
    let customers = helper::load_jsonl::<Customer>("data/noahs-customers.jsonl")?;
    let orders = helper::load_jsonl::<Order>("data/noahs-orders.jsonl")?;
    
    let mut early_bird_counts: HashMap<i64, i32> = HashMap::new();
    for order in orders.iter() {
        let hour = order
            .ordered
            .split_whitespace()
            .nth(1)
            .and_then(|t| t.split(':').next())
            .and_then(|h| h.parse::<i32>().ok())
            .unwrap_or(12);

        if hour >= 0 && hour < 6 {
            if order.items.iter().any(|item| item.sku.contains("BKY")) {
                *early_bird_counts.entry(order.customerid).or_insert(0) += 1;
            }
        }
    }

    let mut early_birds: Vec<(&i64, &i32)> = early_bird_counts.iter().collect();
    early_birds.sort_by(|a, b| b.1.cmp(a.1));

    if let Some((customer_id, _count)) = early_birds.first() {
        if let Some(customer) = customers.iter().find(|c| &c.customerid == *customer_id) {
            helper::print_solution(4, customer.phone.clone());
        }
    }

    Ok(())
}
