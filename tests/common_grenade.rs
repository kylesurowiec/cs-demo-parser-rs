use cs_demo_parser::common::new_grenade_projectile;

#[test]
fn grenade_projectile_unique_id() {
    let a = new_grenade_projectile();
    let b = new_grenade_projectile();
    assert_ne!(a.unique_id, b.unique_id);
}
