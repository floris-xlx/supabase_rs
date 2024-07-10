pub mod base;

pub mod methods {
    pub mod delete;
    pub mod init;
    pub mod insert;
    pub mod insert_if_unique_numeric;
    pub mod insert_if_unique_string;
    pub mod insert_numeric;
    pub mod insert_string;
    pub mod select;
    pub mod select_filter;
    pub mod select_with_columns;
    pub mod select_with_count;
    pub mod select_with_count_and_filter;
    pub mod upsert_numeric;
    pub mod upsert_string;
    pub mod update_with_column;
    pub mod select_stacked_queries;
}