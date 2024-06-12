use anyhow::{ Result, Error as AnyError };
use serde_json::Value;

/// ### Illegal table name
pub fn illegal_table_name(table_name: &str) -> String {
    let amount_of_numbers: usize = table_name
        .chars()
        .take_while(|c| c.is_numeric())
        .count();
    let arrow_amount: String = "^".repeat(amount_of_numbers);

    let is_plural: &str = if amount_of_numbers > 1 { "s" } else { "" };

    let error = format!(
        "\x1b[1;31mYour Query is invalid!!, there is a number as the first character{} in the table name: \n\n\x1b[1;34m{}\x1b[0m\n\x1b[1;34m{}\x1b[0m\n\x1b[1;31mremove these numbers and try again\x1b[0m",
        is_plural,
        table_name,
        arrow_amount
    );

    println!("{}", error);

    "Illegal table name".to_string()
}

pub fn table_does_not_exist(name: &str) -> String {
    let error: String = format!("\x1b[1;31mTable does not exist: {}\x1b[0m", name).to_string();

    println!("{}", error);

    "Table does not exist".to_string()
}


pub fn field_does_not_exist_on_table(field: &str, table: &str) -> String {
    let error: String = format!("\x1b[1;31mField does not exist on table: \n {} -> {}\x1b[0m", table, field).to_string();

    println!("{}", error);

    "Field does not exist on table".to_string()
}

pub fn table_name_does_not_end_with_collection(table_name: &str) -> String {
    let error: String = format!("\x1b[1;31mTable name does not end with \x1b[1;34m`Collection`\x1b[1;31m: {}\x1b[0m", table_name).to_string();
    let arrow_amount: String = "^".repeat(table_name.len());

    println!("{}", error);
    println!("\x1b[1;34m{}\x1b[0m\x1b[1;32mCollection\x1b[0m", table_name);
    println!("\x1b[1;34m{}\x1b[0m", arrow_amount);
    println!("Add Collection to the end of the table name and try again");

    "Table name does not end with Collection".to_string()
}