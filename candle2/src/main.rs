use helper::prelude::*;

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
    let cleaner = "HOM2761";
    let coffee = "DLI8820";
    let sesame_bagel = "BKY1573";
    let caraway_bagel = "BKY5717";

    let customers = helper::load_jsonl::<Customer>("data/noahs-customers.jsonl")?;
    let orders = helper::load_jsonl::<Order>("data/noahs-orders.jsonl")?;

    let order = orders
        .into_iter()
        .filter(|o| o.ordered.starts_with("2017"))
        .filter(|o| {
            let items = o.items.iter().map(|i| i.sku.clone()).collect::<Vec<String>>();
            items.contains(&cleaner.to_string())
                && items.contains(&coffee.to_string())
                && (items.contains(&sesame_bagel.to_string()) || items.contains(&caraway_bagel.to_string()))
        })
        .map(|o| o.customerid)
        .collect::<Vec<i64>>();

    let customer = customers
        .into_iter()
        .filter(|customer| order.contains(&customer.customerid))
        .map(|c| c.phone)
        .collect::<Vec<String>>()
        .join("\n");

    helper::print_solution(2, customer);
    Ok(())
}
