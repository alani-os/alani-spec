#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_observability::repository_name(), "alani-observability");
    assert!(!alani_observability::module_names().is_empty());
}
