#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_memory::repository_name(), "alani-memory");
    assert!(!alani_memory::module_names().is_empty());
}
