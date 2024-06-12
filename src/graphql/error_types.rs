use anyhow::{Result, Error as AnyError};

/// ### Illegal table name
pub fn illegal_table_name(table_name: &str) -> String {
    let amount_of_numbers: usize = table_name.chars().take_while(|c| c.is_numeric()).count();
    let arrow_amount: String = "^".repeat(amount_of_numbers);

    let is_plural: &str = if amount_of_numbers > 1 { "s" } else { "" };

    let error = format!(
        "\x1b[1;31mYour Query is invalid!!, there is a number as the first character{} in the table name: \n\n\x1b[1;34m{}\x1b[0m\n\x1b[1;34m{}\x1b[0m\n\x1b[1;31mremove these numbers and try again\x1b[0m",
        is_plural, table_name, arrow_amount
    );

    println!("{}", error);

    "Illegal table name".to_string()
}
