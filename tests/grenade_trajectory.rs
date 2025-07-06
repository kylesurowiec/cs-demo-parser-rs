use cs_demo_parser::common::{GrenadeProjectile, new_grenade_projectile};
use cs_demo_parser::sendtables::entity::Vector;
use std::time::Duration;

#[test]
fn projectile_tracks_positions() {
    let mut g = new_grenade_projectile();
    g.track_position(
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        1,
        Duration::from_millis(1),
    );
    g.track_position(
        Vector {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        2,
        Duration::from_millis(2),
    );
    assert_eq!(2, g.trajectory.len());
    assert_eq!(
        Some(&Vector {
            x: 1.0,
            y: 1.0,
            z: 0.0
        }),
        g.last_position()
    );
    assert_eq!(2, g.trajectory2.len());
    assert_eq!(2, g.trajectory2[1].frame_id);
}
