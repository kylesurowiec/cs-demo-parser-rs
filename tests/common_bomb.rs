use demoinfocs_rs::common::{Bomb, Player};
use demoinfocs_rs::sendtables::entity::Vector;

#[test]
fn bomb_position() {
    let mut bomb = Bomb::default();
    bomb.last_on_ground_position = Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    assert_eq!(
        bomb.position(),
        Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0
        }
    );

    let carrier = Player {
        last_alive_position: Vector {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        },
        ..Default::default()
    };
    bomb.carrier = Some(carrier);
    assert_eq!(bomb.position(), bomb.carrier.as_ref().unwrap().position());
}
