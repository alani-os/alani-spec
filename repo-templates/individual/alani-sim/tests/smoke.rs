#[test]
fn repository_identity_is_stable() {
    assert_eq!(alani_sim::repository_name(), "alani-sim");
    assert!(!alani_sim::module_names().is_empty());
}
