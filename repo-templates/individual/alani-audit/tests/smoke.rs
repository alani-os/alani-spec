#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_audit::repository_name(), "alani-audit");
    assert!(!alani_audit::module_names().is_empty());
}
