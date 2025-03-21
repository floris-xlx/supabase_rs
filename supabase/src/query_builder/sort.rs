use crate::query::{Sort, SortOrder};

use std::fmt::{Display, Formatter, Result};

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}.{}",
            self.column,
            match self.order {
                SortOrder::Ascending => "asc",
                SortOrder::Descending => "desc",
            }
        )
    }
}
