use demoinfocs_rs::common::convex_hull;
use demoinfocs_rs::sendtables::entity::Vector;

#[test]
fn convex_hull_basic() {
    let points = vec![
        Vector { x: 0.0, y: 0.0, z: 0.0 },
        Vector { x: 1.0, y: 0.0, z: 0.0 },
        Vector { x: 1.0, y: 1.0, z: 0.0 },
        Vector { x: 0.0, y: 1.0, z: 0.0 },
        Vector { x: 0.5, y: 0.5, z: 0.0 },
    ];
    let hull = convex_hull(&points);
    assert_eq!(4, hull.len());
}
