# Payment Client Account - Rust

This is a CLI handling a brief payment client account structure.
It handles data from a CSV and outputs via stdout to a csv file specified.

It can be run via 
<br>
`cargo run -- transactions.csv > accounts.csv`

For this demo i have a fixtures which contains an initial transaction file so that can be run by 
<br>
`cargo run -- fixtures/transactions.csv > accounts.csv`

# Project thoughts

My general consensus was to split this into the two parts that are cared about.
Accounts and Transactions.
<br>
Transactions feed the accounts, In the end I needed to know what transactions were associated to the account,
due to the requirements of a dispute/resolve/chargeback

<br>

# Testing
I used the insta crate along with rstest to test different cases of scenarios that could happen.
This tests a few different scenarios that could occur to ensure that this produces the right resutls.


# Extra Crates

Csv crate to handle csv inputs and outputs.
rust_decimal - to handle decimals in rust and allow rounding to 4 precisions.
