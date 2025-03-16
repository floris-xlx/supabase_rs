use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::str::Chars;
use tokio;
use tokio_postgres::{Client, Config, NoTls};

pub async fn generate_supabase_types() {
    let mut config: Config = Config::new();
    config
        .host("aws-0-eu-central-1.pooler.supabase.com")
        .port(6543)
        .user(&env::var("SUPABASE_USER").expect("SUPABASE_USER must be set"))
        .password(&env::var("SUPABASE_PASSWORD").expect("SUPABASE_PASSWORD must be set"))
        .dbname("postgres");

    let (client, connection) = config
        .connect(NoTls)
        .await
        .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let query: &str = "
        SELECT table_name, column_name, data_type, is_nullable
        FROM information_schema.columns
        WHERE table_schema = 'public'
        ORDER BY table_name, ordinal_position;
    ";

    let mut table_definitions: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut all_columns: HashMap<String, Vec<String>> = HashMap::new();

    for row in client
        .query(query, &[])
        .await
        .expect("Failed to fetch schema")
    {
        let table_name: String = row.get("table_name");
        let column_name: String = row.get("column_name");
        let data_type: String = row.get("data_type");
        let is_nullable: String = row.get("is_nullable");

        let base_rust_type: &str = match data_type.as_str() {
            "integer" => "i32",
            "bigint" => "i64",
            "smallint" => "i16",
            "text" | "varchar" | "char" => "String",
            "boolean" => "bool",
            "real" | "double precision" => "f64",
            "numeric" | "decimal" => "Decimal",
            "timestamp without time zone" | "timestamp with time zone" => "NaiveDateTime",
            "date" => "NaiveDate",
            "uuid" => "Uuid",
            _ => "String",
        };

        let rust_type = if is_nullable == "YES" {
            format!("Option<{}>", base_rust_type)
        } else {
            base_rust_type.to_string()
        };

        table_definitions
            .entry(table_name.clone())
            .or_insert_with(Vec::new)
            .push((column_name.clone(), rust_type));

        // Track column names for the columns method
        all_columns
            .entry(table_name)
            .or_insert_with(Vec::new)
            .push(column_name);
    }

    let mut output: String = String::new();
    output.push_str("use serde::{Serialize, Deserialize};\n\n");
    output.push_str("use chrono::{NaiveDate, NaiveDateTime};\n");
    output.push_str("use uuid::Uuid;\n");
    output.push_str("use rust_decimal::Decimal;\n\n");
    output.push_str("use crate::SupabaseClient;\n");
    output.push_str("use crate::query::QueryBuilder;\n\n");

    let mut all_tables: Vec<String> = Vec::new();

    // Add structs for each table and the select method
    for (table, columns) in &table_definitions {
        all_tables.push(table.clone());
        let struct_name = pascal_case(table);

        // Generate the struct for the table
        output.push_str(&format!(
            "#[derive(Debug, Serialize, Deserialize)]\npub struct {} {{\n",
            struct_name
        ));

        for (col, rust_type) in columns {
            let field_name = if col == "type" {
                "type_".to_string()
            } else {
                col.clone()
            };
            let rename_attr: String = if col == "type" {
                format!("    #[serde(rename = \"{}\")]\n", col)
            } else {
                String::new()
            };
            output.push_str(&rename_attr);
            output.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
        }
        output.push_str("}\n\n");

        // Generate the columns method for this table
        output.push_str(&format!(
            "impl {} {{\n    pub fn columns() -> &'static [&'static str] {{\n",
            struct_name
        ));
        // Add column names dynamically
        let column_names = all_columns.get(table).unwrap();
        output.push_str("        &[\n");
        for col in column_names {
            output.push_str(&format!("            \"{}\",\n", col));
        }
        output.push_str("        ]\n");
        output.push_str("    }\n}\n\n");

        // Generate the select method for this table
        output.push_str(&format!(
            "impl SupabaseClient {{\n    /// ### Columns\n    /// | Column Name | Type | Optional |\n    /// |-------------|------|----------|\n"
        ));

        for (col, rust_type) in columns {
            let optional = if rust_type.contains("Option<") { "Yes" } else { "Required" };
            output.push_str(&format!(
                "    /// | {} | {} | {} |\n",
                col, rust_type.replace("Option<", "").replace(">", ""), optional
            ));
        }

        output.push_str(&format!(
            "    #[cfg(feature = \"nightly\")]\n    pub fn select_{}(&self) -> QueryBuilder {{\n",
            snake_case(&struct_name)
        ));

        output.push_str(&format!(
            "        QueryBuilder::new(self.clone(), \"{}\")\n    }}\n}}\n\n",
            table
        ));
    }

    // Add a part that contains all tables
    output.push_str("pub const ALL_TABLES: &[&str] = &[\n");
    for table in all_tables {
        output.push_str(&format!("    \"{}\",\n", table));
    }
    output.push_str("];\n\n");

    // Write to file
    use std::fs;
    if fs::metadata("src/lib.rs").is_ok() {
        let mut lib_rs: File = OpenOptions::new()
            .read(true)
            .open("src/lib.rs")
            .expect("Failed to open lib.rs");
        let mut contents = String::new();
        lib_rs
            .read_to_string(&mut contents)
            .expect("Failed to read lib.rs");

        if !contents.contains("pub mod supabase_types;") {
            let mut lib_rs: File = OpenOptions::new()
                .write(true)
                .append(true)
                .open("src/lib.rs")
                .expect("Failed to open lib.rs");
            lib_rs
                .write_all(b"pub mod supabase_types;\n")
                .expect("Failed to write to lib.rs");
        }
    } else if fs::metadata("src/mod.rs").is_ok() {
        let mut mod_rs: File = OpenOptions::new()
            .read(true)
            .open("src/mod.rs")
            .expect("Failed to open mod.rs");
        let mut contents = String::new();
        mod_rs
            .read_to_string(&mut contents)
            .expect("Failed to read mod.rs");

        if !contents.contains("pub mod supabase_types;") {
            let mut mod_rs: File = OpenOptions::new()
                .write(true)
                .append(true)
                .open("src/mod.rs")
                .expect("Failed to open mod.rs");
            mod_rs
                .write_all(b"pub mod supabase_types;\n")
                .expect("Failed to write to mod.rs");
        }
    }

    if fs::metadata("src/supabase_types.rs").is_ok() {
        fs::remove_file("src/supabase_types.rs")
            .expect("Failed to remove existing supabase_types.rs");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("src/supabase_types.rs")
        .expect("Failed to open or create file");
    file.write_all(output.as_bytes())
        .expect("Failed to write file");
}

fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars: Chars<'_> = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>()
}

fn snake_case(s: &str) -> String {
    let mut result: String = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}
