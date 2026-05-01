#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_runtime::repository_name(), "alani-runtime");
    assert!(!alani_runtime::module_names().is_empty());
}
