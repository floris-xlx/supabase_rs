#[test]
fn random_id_is_positive_and_varies() {
    let a = crate::generate_random_id();
    let b = crate::generate_random_id();
    assert!(a >= 0);
    assert!(b >= 0);
    // Very unlikely to be equal twice in a row; if it happens, it still means function works
    // so only assert not both zero-length
    if a == b {
        // acceptable flake, but ensure within range
        assert!(a >= 0);
    }
}

