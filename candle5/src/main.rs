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
struct Order {
    pub orderid: i64,
    pub customerid: i64,
    pub ordered: String,
    pub shipped: String,
    pub items: Vec<Item>,
    pub total: f64,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Item {
    pub sku: String,
    pub qty: i64,
    pub unit_price: f64,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Product {
    pub sku: String,
    pub desc: String,
    pub wholesale_cost: f64,
    pub dims_cm: Vec<f64>,
}

fn main() -> Result<()> {
    let customers = helper::load_jsonl::<Customer>("data/noahs-customers.jsonl")?;
    let orders = helper::load_jsonl::<Order>("data/noahs-orders.jsonl")?;
    let products = helper::load_jsonl::<Product>("data/noahs-products.jsonl")?;

    let product_map: HashMap<String, String> = products.into_par_iter().map(|p| (p.sku, p.desc)).collect();
    let orders_by_customer: HashMap<i64, Vec<&Order>> = orders
        .par_iter()
        .fold(
            || HashMap::<i64, Vec<&Order>>::new(),
            |mut acc, order| {
                acc.entry(order.customerid).or_default().push(order);
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut a, b| {
                for (k, v) in b {
                    a.entry(k).or_default().extend(v);
                }
                a
            },
        );

    // Lookup Staten Island customers
    let cat_lady = customers
        .par_iter()
        .filter(|c| c.citystatezip.contains("Staten Island"))
        .filter_map(|customer| {
            let customer_orders = orders_by_customer.get(&customer.customerid)?;

            // "But it was still really dirty, so when I was going through a Marie Kondo phase, I decided it wasn’t sparking joy anymore."
            // Marie Kondo had a Netflix show back in 2019, so we check for that, cool
            let has_early_2019_orders = customer_orders
                .iter()
                .any(|o| o.ordered.starts_with("2019-01") || o.ordered.starts_with("2019-02") || o.ordered.starts_with("2019-03"));

            if !has_early_2019_orders {
                return None;
            }

            // Senior cat food orders
            let senior_cat_food_qty: i64 = customer_orders
                .iter()
                .flat_map(|o| &o.items)
                .filter_map(|item| {
                    let desc = product_map.get(&item.sku)?;
                    if desc.to_lowercase().contains("senior") && desc.to_lowercase().contains("cat") {
                        Some(item.qty)
                    } else {
                        None
                    }
                })
                .sum();

            // We're looking for someone who:
            // 1. Orders lots of senior cat food (suggesting 10-11 old cats)
            // 2. Orders in bulk (7+ items per order)
            // 3. Was active in early 2019, during the Marie Kondo phase
            if senior_cat_food_qty >= 50 {
                Some((customer, senior_cat_food_qty))
            } else {
                None
            }
        })
        .max_by_key(|&(_, qty)| qty)
        .map(|(customer, _)| customer.phone.clone());

    if let Some(phone) = cat_lady {
        helper::print_solution(5, phone);
    }

    Ok(())
}
