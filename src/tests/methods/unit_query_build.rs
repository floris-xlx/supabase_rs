#[tokio::test]
async fn add_param_deduplicates() {
    let mut q = crate::query::Query::new();
    q.add_param("limit", "10");
    q.add_param("limit", "10");
    // Duplicate should not be added twice
    let built = q.build();
    assert!(built == "limit=10" || built == "limit=10&");
}

#[tokio::test]
async fn build_orders_params_and_filters() {
    let mut q = crate::query::Query::new();
    q.add_param("select", "id,name");
    q.add_param("limit", "10");
    let s = q.build();
    // Order between params is insertion order in implementation; we just assert both keys exist
    assert!(s.contains("select=id,name"));
    assert!(s.contains("limit=10"));
}
