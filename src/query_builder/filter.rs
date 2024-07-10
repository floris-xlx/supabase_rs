use crate::query::Filter;
use crate::query::Operator;

use std::fmt::{Display, Formatter, Result};

impl Filter {
    /// Constructs a new `Filter` instance.
    ///
    /// # Arguments
    /// * `column` - A `String` specifying the column name to which the filter will apply.
    /// * `operator` - An `Operator` enum specifying the type of comparison (e.g., Equals, NotEquals).
    /// * `value` - A `String` representing the value to compare against the column.
    ///
    /// # Returns
    /// Returns a new `Filter` instance containing the specified column, operator, and value.
    ///
    /// # Examples
    /// ```
    /// let filter = Filter::new("age".to_string(), Operator::GreaterThan, "30".to_string());
    /// ```
    pub fn new(column: String, operator: Operator, value: String) -> Filter {
        Filter {
            column,
            operator,
            value,
        }
    }
}

impl Display for Filter {
    /// Converts the filter into a query string format.
    ///
    /// This method formats the filter's column, operator, and value into a string
    /// that can be used directly in a query URL. The operator is converted to its
    /// corresponding string representation (`eq`, `neq`, `gt`, `lt`, `gte`, `lte`).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use supabase_rs::query::{Filter, Operator};
    /// let filter = Filter::new("age".to_string(), Operator::GreaterThan, "30".to_string());
    /// assert_eq!(filter.to_string(), "age.gt=30");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}.{}={}",
            self.column,
            match self.operator {
                Operator::Equals => "eq",
                Operator::NotEquals => "neq",
                Operator::GreaterThan => "gt",
                Operator::LessThan => "lt",
                Operator::GreaterThanOrEquals => "gte",
                Operator::LessThanOrEquals => "lte",
            },
            self.value
        )
    }
}
