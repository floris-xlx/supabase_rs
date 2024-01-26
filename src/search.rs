use reqwest;
use serde_json::Value;
use reqwest::Client;

use crate::SupabaseClient;

// here we're gonna construct the request for the update
// building the Query builder


pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals
}

pub enum SortOrder {
    Ascending,
    Descending
}

pub struct Filter {
    column: String,
    operator: Operator,
    value: String
}

pub struct Sort {
    column: String,
    order: SortOrder
}

pub struct Query {
    filters: Vec<Filter>,
    sort: Option<Sort>,
    limit: Option<u32>,
    offset: Option<u32>
}

impl Filter {
    // constructs a new filter
    pub fn new(
        column: String,
        operator: Operator,
        value: String
    ) -> Filter {
        Filter {
            column,
            operator,
            value
        }
    }

    // converts the filter to a string
    pub fn to_string(
        &self
    ) -> String {
        format!(
            "{}.{}={}",
            self.column,
            match self.operator {
                Operator::Equals => "eq",
                Operator::NotEquals => "neq",
                Operator::GreaterThan => "gt",
                Operator::LessThan => "lt",
                Operator::GreaterThanOrEquals => "gte",
                Operator::LessThanOrEquals => "lte"
            },
            self.value
        )
    }
}

// implementation of the query builder
impl Query {
    // constructs a new query

    pub fn new() -> Query {
        Query {
            filters: Vec::new(),
            sort: None,
            limit: None,
            offset: None
        }
    }

    // method to add filter
    pub fn filter(
        mut self,
        filter: Filter
    ) -> Query {
        self.filters.push(filter);
        self
    }

    // method to set sorter
    pub fn sort (
        mut self,
        sort: Sort
    ) -> Query {
        self.sort = Some(sort);
        self
    }

    // method to set limit
    pub fn limit (
        mut self,
        limit: u32
    ) -> Query {
        self.limit = Some(limit);
        self
    }

    // method to set offset
    pub fn offset (
        mut self,
        offset: u32
    ) -> Query {
        self.offset = Some(offset);
        self
    }

    // method to build the query string
    pub fn build(
        self
    ) -> String {
        let mut query_string: String = String::new();

        // add filters
        if self.filters.len() > 0 {
            query_string.push_str("?");
            for (index, filter) in self.filters.iter().enumerate() {
                query_string.push_str(&filter.to_string());
                if index < self.filters.len() - 1 {
                    query_string.push_str("&");
                }
            }
        }

        // add sort
        if let Some(sort) = self.sort {
            query_string.push_str(&format!(
                "{}{}",
                if self.filters.len() > 0 { "&" } else { "?" },
                sort.to_string()
            ));
        }

        // add limit
        if let Some(limit) = self.limit {
            query_string.push_str(&format!(
                "{}limit={}",
                if self.filters.len() > 0 || self.sort.is_some() { "&" } else { "?" },
                limit
            ));
        }

        // add offset
        if let Some(offset) = self.offset {
            query_string.push_str(&format!(
                "{}offset={}",
                if self.filters.len() > 0 || self.sort.is_some() || self.limit.is_some() { "&" } else { "?" },
                offset
            ));
        }

        query_string
    }
}


// build the select function with granularity
impl SupabaseClient {
    pub async fn select(
        &self,
        table_name: &str,
        search_column: &str,
        search_query: &str
    ) -> Result<Vec<Value>, String> {
        // build the client and the endpoints

        let endpoint: String = format!(
            "{}/rest/v1/{}",
            self.url, table_name
        );
        let client: Client = Client::new();
        let query: Query = Query::new()
            .filter(
                Filter::new(
                    search_column.to_string(),
                    Operator::Equals,
                    search_query.to_string()
                )
        );

        let response: Response = match client
            .get(&endpoint)
            .header("apikey", &self.api_key)
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .query(&query.build())
            .send()
            .await {
                Ok(response) => response,
                Err(error) => return Err(error.to_string())
            };

        if response.status().is_success() {
            let records: Result<Vec<Value>, _> = match response.json().await {
                Ok(records) => Ok(records),
                Err(error) => Err(error.to_string())
            };
            return records

        } else {
            return Err(response.status().to_string())
        }
    }

}