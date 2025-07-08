use cruet::string::singularize::to_singular;
use std::collections::BTreeMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::str::Chars;
use tokio;
use tokio_postgres::{Config, NoTls};

pub async fn generate_supabase_types(
    user: &str,
    password: &str,
    singularize_struct_name: bool,
    excluded_tables: &[&str],
) {
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
        SELECT
            table_name,
            column_name,
            data_type,
            is_nullable,
            column_default,
            is_identity
        FROM information_schema.columns
        WHERE table_schema = 'public'
        ORDER BY table_name, ordinal_position;
    ";

    let mut table_definitions: BTreeMap<String, Vec<(String, String, bool, bool)>> =
        BTreeMap::new();
    let mut all_columns: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for row in client.query(query, &[]).await.expect("query") {
        let table_name: String = row.get("table_name");
        if excluded_tables.contains(&table_name.as_str()) {
            continue;
        }
        let column_name: String = row.get("column_name");
        let data_type: String = row.get("data_type");
        let is_nullable: String = row.get("is_nullable");
        let column_default: Option<String> = row.get("column_default");
        let is_identity: String = row.get("is_identity");

        let base_rust_type: &'static str = match data_type.as_str() {
            "integer" => "i32",
            "bigint" => "i64",
            "smallint" => "i16",
            "text" | "varchar" | "char" => "String",
            "boolean" => "bool",
            "real" | "double precision" => "f64",
            "numeric" | "decimal" => "Decimal",
            "timestamp without time zone" => "NaiveDateTime",
            "timestamp with time zone" => "DateTime<Utc>",
            "date" => "NaiveDate",
            "uuid" => "Uuid",
            "json" | "jsonb" => "Value",
            _ => "String",
        };

        // only nullable columns become Option in the primary struct
        let nullable_flag: bool = is_nullable == "YES";
        // default_flag still needed for New<> below
        let default_flag: bool = is_identity == "YES" || column_default.is_some();
        // **Primary** type uses only nullable_flag
        let rust_type: String = if nullable_flag {
            format!("Option<{base_rust_type}>")
        } else {
            base_rust_type.to_string()
        };

        // stash both the primary type and flags for use in New<>
        table_definitions
            .entry(table_name.clone())
            .or_default()
            .push((column_name.clone(), rust_type, nullable_flag, default_flag));

        all_columns.entry(table_name).or_default().push(column_name);
    }

    let mut output: String = String::new();
    output.push_str("#![allow(dead_code)]\n\n");
    output.push_str("use serde::{Serialize, Deserialize};\n\n");
    output.push_str("use serde_json::Value;\n\n");
    output.push_str("use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime};\n");
    output.push_str("use uuid::Uuid;\n");
    output.push_str("use rust_decimal::Decimal;\n\n");
    output.push_str("use supabase_rs::SupabaseClient;\n");
    output.push_str("use supabase_rs::query::QueryBuilder;\n\n");

    let mut all_tables: Vec<String> = Vec::new();
    let mut trait_methods: String = String::new();
    let mut impl_methods: String = String::new();

    // ensure tables are emitted in sorted order:
    let mut tables: Vec<_> = table_definitions.keys().cloned().collect();
    tables.sort();
    for table in &tables {
        let columns: &Vec<(String, String, bool, bool)> = &table_definitions[table];
        let struct_name: String = if singularize_struct_name {
            pascal_case(&to_singular(table))
        } else {
            pascal_case(table)
        };
        all_tables.push(table.clone());

        if singularize_struct_name {
            output.push_str(&format!("// table name: {}\n", table));
        }

        // — Primary struct
        output.push_str(&format!(
            "#[derive(Debug, Serialize, Deserialize, Clone)]\n\
             pub struct {struct_name} {{\n"
        ));
        for (col, ty, _, _) in columns {
            let field: String = safe_field_name(col);
            if &field != col {
                output.push_str(&format!("    #[serde(rename = \"{col}\")]\n"));
            }
            output.push_str(&format!("    pub {field}: {ty},\n"));
        }
        output.push_str("}\n\n");

        // — New<T> struct
        let new_name: String = format!("New{struct_name}");
        output.push_str(&format!(
            "#[derive(Debug, Serialize, Deserialize, Clone, Default)]\n\
             pub struct {new_name} {{\n"
        ));
        for (col, ty, nullable, default) in columns {
            let field: String = safe_field_name(col);
            // unwrap Option<…>
            let inner: &str = ty
                .strip_prefix("Option<")
                .and_then(|s| s.strip_suffix('>'))
                .unwrap_or(&ty);
            if &field != col {
                output.push_str(&format!("    #[serde(rename = \"{col}\")]\n"));
            }
            if *nullable || *default {
                output.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                output.push_str(&format!("    pub {field}: Option<{inner}>,\n"));
            } else {
                output.push_str(&format!("    pub {field}: {inner},\n"));
            }
        }
        output.push_str("}\n\n");

        // — columns() fn
        output.push_str(&format!(
            "impl {struct_name} {{\n    pub fn columns() -> &'static [&'static str] {{\n"
        ));
        output.push_str("        &[\n");
        for col in &all_columns[table] {
            output.push_str(&format!("            \"{col}\",\n"));
        }
        output.push_str("        ]\n    }\n\n");
        output.push_str(&format!("    pub fn table_name() -> &'static str {{ \"{}\" }}\n", table));
        output.push_str("}\n\n");

        // — extension trait methods
        trait_methods.push_str(&format!("    fn select_{table}(&self) -> QueryBuilder;\n"));
        impl_methods.push_str(&format!(
            "    fn select_{table}(&self) -> QueryBuilder {{\n        QueryBuilder::new(self.clone(), \"{table}\")\n    }}\n"
        ));
    }

    // ALL_TABLES constant
    output.push_str("pub const ALL_TABLES: &[&str] = &[\n");
    for t in &all_tables {
        output.push_str(&format!("    \"{t}\",\n"));
    }
    output.push_str("];\n\n");

    // emit extension trait + impl
    output.push_str("pub trait SupabaseClientExt {\n");
    output.push_str(&trait_methods);
    output.push_str("}\n\n");
    output.push_str("impl SupabaseClientExt for SupabaseClient {\n");
    output.push_str(&impl_methods);
    output.push_str("}\n");

    if fs::metadata("src/lib.rs").is_ok() {
        let mut lib_rs: File = OpenOptions::new().read(true).open("src/lib.rs").unwrap();
        let mut contents: String = String::new();
        lib_rs.read_to_string(&mut contents).unwrap();
        if !contents.contains("pub mod supabase_types;") {
            let mut lib_rs: File = OpenOptions::new()
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

    // write file
    if fs::metadata("src/supabase_types.rs").is_ok() {
        fs::remove_file("src/supabase_types.rs").unwrap();
    }
    let mut file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .open("src/supabase_types.rs")
        .unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c: Chars<'_> = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn snake_case(s: &str) -> String {
    let mut out: String = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_lowercase().next().unwrap());
    }
    out
}

fn safe_field_name(col: &str) -> String {
    if col == "type" {
        "type_".into()
    } else if col.chars().all(|c| c.is_ascii_uppercase()) {
        col.to_lowercase()
    } else {
        snake_case(col)
    }
}
