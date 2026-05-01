#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_security::repository_name(), "alani-security");
    assert!(!alani_security::module_names().is_empty());
}
