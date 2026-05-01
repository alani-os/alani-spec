#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_package::repository_name(), "alani-package");
    assert!(!alani_package::module_names().is_empty());
}
