use cs_demo_parser::parser::Parser;
use cs_demo_parser::proto::{msg, msgs2};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

fn args() -> (String, String) {
    let mut args = env::args().skip(1);
    let mut demo = None;
    let mut out = String::from("voice.raw");
    while let Some(arg) = args.next() {
        match arg.as_str() {
            | "-demo" => demo = args.next(),
            | "-out" => out = args.next().expect("-out requires a path after it"),
            | _ => {
                eprintln!("Usage: cargo run --example voice_capture -- -demo <demo> [-out <file>]");
                std::process::exit(1);
            },
        }
    }
    let demo = demo.expect("-demo <demo> is required");
    (demo, out)
}

fn main() {
    let (demo_path, out_path) = args();

    let file = File::open(&demo_path).expect("failed to open demo file");
    let mut parser = Parser::new(file);

    let writer = Arc::new(Mutex::new(BufWriter::new(
        File::create(&out_path).expect("failed to create output file"),
    )));

    {
        let writer = Arc::clone(&writer);
        parser.register_net_message_handler::<msg::CsvcMsgVoiceData, _>(move |msg| {
            if let Some(data) = msg.voice_data.as_ref() {
                if let Ok(mut w) = writer.lock() {
                    let _ = w.write_all(data);
                }
            }
        });
    }

    {
        let writer = Arc::clone(&writer);
        parser.register_net_message_handler::<msgs2::CsvcMsgVoiceData, _>(move |msg| {
            if let Some(audio) = &msg.audio {
                if let Some(data) = audio.voice_data.as_ref() {
                    if let Ok(mut w) = writer.lock() {
                        let _ = w.write_all(data);
                    }
                }
            }
        });
    }

    if let Err(e) = parser.parse_to_end() {
        eprintln!("error while parsing demo: {:?}", e);
    }

    println!("voice data written to {}", out_path);
}
