#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_filesystem::repository_name(), "alani-filesystem");
    assert!(!alani_filesystem::module_names().is_empty());
}
