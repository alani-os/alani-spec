#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_tests::repository_name(), "alani-tests");
    assert!(!alani_tests::module_names().is_empty());
}
