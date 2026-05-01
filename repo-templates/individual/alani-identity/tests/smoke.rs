#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_identity::repository_name(), "alani-identity");
    assert!(!alani_identity::module_names().is_empty());
}
