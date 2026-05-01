#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_userspace::repository_name(), "alani-userspace");
    assert!(!alani_userspace::module_names().is_empty());
}
