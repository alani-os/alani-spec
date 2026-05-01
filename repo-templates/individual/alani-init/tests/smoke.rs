#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_init::repository_name(), "alani-init");
    assert!(!alani_init::module_names().is_empty());
}
