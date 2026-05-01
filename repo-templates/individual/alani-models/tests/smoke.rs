#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_models::repository_name(), "alani-models");
    assert!(!alani_models::module_names().is_empty());
}
