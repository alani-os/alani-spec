#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_storage::repository_name(), "alani-storage");
    assert!(!alani_storage::module_names().is_empty());
}
