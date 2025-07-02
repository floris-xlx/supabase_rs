use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::str::Chars;
use tokio;
use tokio_postgres::{Config, NoTls};

pub async fn generate_supabase_types(user: &str, password: &str) {
    // connect to your supabase Postgres pooler
    let mut config: Config = Config::new();
    config
        .host("aws-0-eu-central-1.pooler.supabase.com")
        .port(6543)
        .user(user)
        .password(password)
        .dbname("postgres");

    let (client, connection) = config
        .connect(NoTls)
        .await
        .expect("Failed to connect to database");

    // spawn the connection driver
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {e}");
        }
    });

    // fetch schema info
    let query: &'static str = "
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

        let base_rust_type: &'static str = match data_type.as_str() {
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
            "json" | "jsonb" => "Value",
            _ => "String",
        };

        let rust_type: String = if is_nullable == "YES" {
            format!("Option<{base_rust_type}>")
        } else {
            base_rust_type.to_string()
        };

        table_definitions
            .entry(table_name.clone())
            .or_insert_with(Vec::new)
            .push((column_name.clone(), rust_type.clone()));

        all_columns
            .entry(table_name)
            .or_insert_with(Vec::new)
            .push(column_name);
    }

    // start building the file
    let mut output: String = String::new();
    output.push_str("#![allow(dead_code)]\n\n");
    output.push_str("use serde::{Serialize, Deserialize};\n\n");
    output.push_str("use serde_json::Value;\n\n");
    output.push_str("use chrono::{NaiveDate, NaiveDateTime};\n");
    output.push_str("use uuid::Uuid;\n");
    output.push_str("use rust_decimal::Decimal;\n\n");
    output.push_str("use supabase_rs::SupabaseClient;\n");
    output.push_str("use supabase_rs::query::QueryBuilder;\n\n");

    let mut all_tables: Vec<String> = Vec::new();
    let mut trait_methods: String = String::new();
    let mut impl_methods: String = String::new();

    for (table, columns) in &table_definitions {
        let struct_name: String = pascal_case(table);
        all_tables.push(table.clone());

        // struct
        output.push_str(&format!(
            "#[derive(Debug, Serialize, Deserialize)]\npub struct {} {{\n",
            struct_name
        ));
        for (col, rust_type) in columns {
            let field_name: String = if col == "type" {
                "type_".to_string()
            } else {
                col.clone()
            };
            let rename: String = if col == "type" {
                format!("    #[serde(rename = \"{col}\")]\n")
            } else {
                String::new()
            };
            output.push_str(&rename);
            output.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
        }
        output.push_str("}\n\n");

        // columns fn
        output.push_str(&format!(
            "impl {} {{\n    pub fn columns() -> &'static [&'static str] {{\n",
            struct_name
        ));
        output.push_str("        &[\n");
        for col in &all_columns[table] {
            output.push_str(&format!("            \"{}\",\n", col));
        }
        output.push_str("        ]\n    }\n}\n\n");

        // extension trait entries
        let snake: String = snake_case(&struct_name);
        trait_methods.push_str(&format!(
            "    fn select_{}(&self) -> QueryBuilder;\n",
            snake
        ));
        impl_methods.push_str(&format!(
            "    fn select_{}(&self) -> QueryBuilder {{\n        QueryBuilder::new(self.clone(), \"{}\")\n    }}\n",
            snake, table
        ));
    }

    // ALL_TABLES constant
    output.push_str("pub const ALL_TABLES: &[&str] = &[\n");
    for table in &all_tables {
        output.push_str(&format!("    \"{}\",\n", table));
    }
    output.push_str("];\n\n");

    // emit extension trait + impl
    output.push_str("pub trait SupabaseClientExt {\n");
    output.push_str(&trait_methods);
    output.push_str("}\n\n");
    output.push_str("impl SupabaseClientExt for SupabaseClient {\n");
    output.push_str(&impl_methods);
    output.push_str("}\n\n");

    // write to file & update mod declaration
    if fs::metadata("src/lib.rs").is_ok() {
        let mut lib_rs: File = OpenOptions::new().read(true).open("src/lib.rs").unwrap();
        let mut contents: String = String::new();
        lib_rs.read_to_string(&mut contents).unwrap();
        if !contents.contains("pub mod supabase_types;") {
            let mut lib_rs = OpenOptions::new()
                .write(true)
                .append(true)
                .open("src/lib.rs")
                .unwrap();
            lib_rs.write_all(b"pub mod supabase_types;\n").unwrap();
        }
    } else if fs::metadata("src/mod.rs").is_ok() {
        let mut mod_rs: File = OpenOptions::new().read(true).open("src/mod.rs").unwrap();
        let mut contents: String = String::new();
        mod_rs.read_to_string(&mut contents).unwrap();
        if !contents.contains("pub mod supabase_types;") {
            let mut mod_rs: File = OpenOptions::new()
                .write(true)
                .append(true)
                .open("src/mod.rs")
                .unwrap();
            mod_rs.write_all(b"pub mod supabase_types;\n").unwrap();
        }
    }

    if fs::metadata("src/supabase_types.rs").is_ok() {
        fs::remove_file("src/supabase_types.rs")
            .expect("Failed to remove existing supabase_types.rs");
    }

    let mut file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("src/supabase_types.rs")
        .unwrap();
    file.write_all(output.as_bytes()).unwrap();
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
        .collect()
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
