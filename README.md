# 🕯️ Hanukkah of Data 5784

Solutions for the [Hanukkah of Data](https://hanukkah.bluebird.sh/5784) puzzles.

## Intro

Convert Jan 1, 2017 (clues: "...set up at the start of 2017..." and "...it’s just the year in the Hebrew calendar when Alex set up the database...") to a Hebrew date.

<details>
    <summary>Answer:</summary>
    <pre>5777</pre>
</details>

## Day 1

Pretty simple. Compare each customer's last name (converted to numbers) to phone number (dropping dashes). Find the one that matches.

<details>
    <summary>Answer:</summary>
    <pre>826-636-2286</pre>
</details>

## Day 2

We're finding orders that contain bagels, coffee, and cleaning supplies.

There's two SKUs for bagels, one for coffee, and one for Rug Cleaner:

```shell
λ cat data/noahs-products.jsonl | rg -i bagel
{"sku": "BKY1573", "desc": "Sesame Bagel", "wholesale_cost": 1.02, "dims_cm": [11.9, 4.7, 0.9]}
{"sku": "BKY5717", "desc": "Caraway Bagel", "wholesale_cost": 1.03, "dims_cm": [11.3, 2.3, 1.6]}
λ cat data/noahs-products.jsonl | rg -i coffee
{"sku": "DLI8820", "desc": "Coffee, Drip", "wholesale_cost": 1.44, "dims_cm": [9.6, 7.8, 0.7]}
λ cat data/noahs-products.jsonl | rg -i clean
{"sku": "HOM2761", "desc": "Rug Cleaner", "wholesale_cost": 1.43, "dims_cm": [19.6, 11.7, 0.2]}
```

Find all of the orders that have those 3 things (coffee, cleaner, and either of the bagels), then map the order's customer ID to the customer record.

<details>
    <summary>Answer:</summary>
    <pre>332-274-4185</pre>
</details>

## Day 3

We're finding someone with a Cancer sign (June 21st to July 22nd) born in the year of the Rabbit (dates below). [Both of these ranges are pulled from Wikipedia](<https://en.wikipedia.org/wiki/Rabbit_(zodiac)#Years_and_elements>).

| Start date       | End date         |
| ---------------- | ---------------- |
| 29 January 1903  | 15 February 1904 |
| 14 February 1915 | 3 February 1916  |
| 2 February 1927  | 22 January 1928  |
| 19 February 1939 | 7 February 1940  |
| 6 February 1951  | 26 January 1952  |
| 25 January 1963  | 12 February 1964 |
| 11 February 1975 | 30 January 1976  |
| 29 January 1987  | 16 February 1988 |
| 16 February 1999 | 4 February 2000  |
| 3 February 2011  | 22 January 2012  |
| 22 January 2023  | 9 February 2024  |
| 8 February 2035  | 27 January 2036  |
| 26 January 2047  | 13 February 2048 |
| 11 February 2059 | 1 February 2060  |
| 31 January 2071  | 18 February 2072 |
| 17 February 2083 | 5 February 2084  |
| 5 February 2095  | 24 January 2096  |

The person is from the same neighborhood as the contract cleaner, so their city should be `Jamaica`.

Filter the customers down based on this information.

<details>
    <summary>Answer:</summary>
    <pre>917-288-9635</pre>
</details>
