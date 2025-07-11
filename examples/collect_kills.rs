use cs_demo_parser::parser::Parser;
use cs_demo_parser::proto::msg::cs_demo_parser_rs::{
    CsvcMsgGameEvent, CsvcMsgGameEventList, csvc_msg_game_event,
};
use std::collections::HashMap;
use std::env;
use std::fs::File;

#[derive(Default)]
struct EventDescriptor {
    name: String,
    keys: Vec<(String, i32)>,
}

fn demo_path_from_args() -> String {
    let mut args = env::args().skip(1);
    match (args.next(), args.next()) {
        | (Some(flag), Some(path)) if flag == "-demo" => path,
        | _ => panic!("Usage: cargo run --example collect_kills -- -demo <path>"),
    }
}

fn key_to_i32(key: &csvc_msg_game_event::KeyT) -> i32 {
    key.val_long.or(key.val_short).or(key.val_byte).unwrap_or(0)
}

fn main() {
    let path = demo_path_from_args();
    let file = File::open(&path).expect("failed to open demo");
    let mut parser = Parser::new(file);

    let descriptors: std::sync::Arc<std::sync::Mutex<HashMap<i32, EventDescriptor>>> =
        Default::default();
    let desc_clone = descriptors.clone();
    parser.register_net_message_handler::<CsvcMsgGameEventList, _>(move |list| {
        let mut map = desc_clone.lock().unwrap();
        map.clear();
        for d in &list.descriptors {
            if let (Some(id), Some(name)) = (d.eventid, d.name.as_ref()) {
                let keys = d
                    .keys
                    .iter()
                    .filter_map(|k| {
                        if let (Some(typ), Some(kn)) = (k.r#type, k.name.as_ref()) {
                            Some((kn.clone(), typ))
                        } else {
                            None
                        }
                    })
                    .collect();
                map.insert(
                    id,
                    EventDescriptor {
                        name: name.clone(),
                        keys,
                    },
                );
            }
        }
    });

    let scoreboard: std::sync::Arc<std::sync::Mutex<HashMap<i32, (u32, u32)>>> = Default::default();
    let names: std::sync::Arc<std::sync::Mutex<HashMap<i32, String>>> = Default::default();
    let desc_clone = descriptors.clone();
    let sb_clone = scoreboard.clone();
    let names_clone = names.clone();
    parser.register_net_message_handler::<CsvcMsgGameEvent, _>(move |ev| {
        let map = desc_clone.lock().unwrap();
        let desc = match ev.eventid.and_then(|id| map.get(&id)) {
            | Some(d) => d,
            | None => return,
        };
        match desc.name.as_str() {
            | "player_connect" | "player_connect_full" => {
                let mut userid = 0;
                let mut name = String::new();
                for ((kname, _), key) in desc.keys.iter().zip(&ev.keys) {
                    match kname.as_str() {
                        | "userid" => userid = key_to_i32(key),
                        | "name" => name = key.val_string.clone().unwrap_or_default(),
                        | _ => {},
                    }
                }
                if userid != 0 && !name.is_empty() {
                    names_clone.lock().unwrap().insert(userid, name);
                }
            },
            | "player_death" => {
                let mut victim = 0;
                let mut attacker = 0;
                for ((kname, _), key) in desc.keys.iter().zip(&ev.keys) {
                    match kname.as_str() {
                        | "userid" => victim = key_to_i32(key),
                        | "attacker" => attacker = key_to_i32(key),
                        | _ => {},
                    }
                }
                if victim != 0 {
                    sb_clone.lock().unwrap().entry(victim).or_insert((0, 0)).1 += 1;
                }
                if attacker != 0 {
                    sb_clone.lock().unwrap().entry(attacker).or_insert((0, 0)).0 += 1;
                }
                let names_map = names_clone.lock().unwrap();
                let attacker_name = names_map
                    .get(&attacker)
                    .cloned()
                    .unwrap_or_else(|| format!("#{}", attacker));
                let victim_name = names_map
                    .get(&victim)
                    .cloned()
                    .unwrap_or_else(|| format!("#{}", victim));
                println!("{} killed {}", attacker_name, victim_name);
            },
            | _ => {},
        }
    });

    if let Err(e) = parser.parse_to_end() {
        eprintln!("error: {:?}", e);
        return;
    }

    let sb = scoreboard.lock().unwrap();
    let gs = parser.game_state();
    println!("scoreboard:");
    for (id, (k, d)) in sb.iter() {
        let name = gs
            .participants()
            .by_user_id()
            .get(id)
            .map(|p| p.name.clone())
            .or_else(|| names.lock().unwrap().get(id).cloned())
            .unwrap_or_else(|| format!("#{}", id));
        println!("{}: {} kills / {} deaths", name, k, d);
    }
}
