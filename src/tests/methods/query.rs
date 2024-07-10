// written by @izyuumi
use crate::query::Query;
use crate::query::Filter;
use crate::query::Operator;
use crate::query::Sort;
use crate::query::SortOrder;


pub async fn test_query() {
    let mut query: Query = Query::new();
    let filter: Filter = Filter {
        column: "age".to_string(),
        operator: Operator::GreaterThan,
        value: "30".to_string(),
    };
    let sort: Sort = Sort {
        column: "name".to_string(),
        order: SortOrder::Ascending,
    };
    query.add_filter(filter);
    query.add_sort(sort);
    let query_string: String = query.build();
    assert_eq!(query_string, "age.gt=30&name.asc");
}