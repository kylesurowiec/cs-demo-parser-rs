use cs_demo_parser::common::Inferno;
use cs_demo_parser::common::convex_hull;
use cs_demo_parser::sendtables::entity::Vector;

#[test]
fn convex_hull_basic() {
    let points = vec![
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vector {
            x: 0.5,
            y: 0.5,
            z: 0.0,
        },
    ];
    let hull = convex_hull(&points);
    assert_eq!(4, hull.len());
}

#[test]
fn inferno_add_flame_updates_hull() {
    let mut i = Inferno::default();
    i.add_flame(Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
    assert_eq!(1, i.hull().len());
    i.add_flame(Vector {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    });
    i.add_flame(Vector {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    });
    assert_eq!(3, i.hull().len());
}
