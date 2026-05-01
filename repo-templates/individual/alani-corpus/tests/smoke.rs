#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_corpus::repository_name(), "alani-corpus");
    assert!(!alani_corpus::module_names().is_empty());
}
