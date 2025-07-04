use demoinfocs_rs::parser::{EntityEvent, Parser};
use demoinfocs_rs::sendtables::EntityOp;
use demoinfocs_rs::sendtables2::{Class, Entity};
use std::io::Cursor;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[test]
fn test_entity_handlers() {
    let mut p = Parser::new(Cursor::new(Vec::<u8>::new()));

    let on_entity = Arc::new(AtomicUsize::new(0));
    let on_created = Arc::new(AtomicUsize::new(0));

    let oe_c = on_entity.clone();
    p.register_on_entity(move |_| {
        oe_c.fetch_add(1, Ordering::SeqCst);
    });
    let oc_c = on_created.clone();
    p.register_on_entity_created(move |_| {
        oc_c.fetch_add(1, Ordering::SeqCst);
    });

    let ent = Entity {
        index: 1,
        serial: 1,
        class: Class {
            class_id: 0,
            name: "Test".into(),
            serializer: None,
        },
    };

    p.dispatch_event(EntityEvent {
        entity: ent.clone(),
        op: EntityOp::CREATED,
    });
    p.dispatch_event(EntityEvent {
        entity: ent.clone(),
        op: EntityOp::UPDATED,
    });

    thread::sleep(std::time::Duration::from_millis(10));

    assert_eq!(2, on_entity.load(Ordering::SeqCst));
    assert_eq!(1, on_created.load(Ordering::SeqCst));
    assert!(p.game_state().entities().get(&1).is_some());
}
