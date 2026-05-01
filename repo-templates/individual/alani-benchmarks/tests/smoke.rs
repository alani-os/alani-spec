#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_benchmarks::repository_name(), "alani-benchmarks");
    assert!(!alani_benchmarks::module_names().is_empty());
}
