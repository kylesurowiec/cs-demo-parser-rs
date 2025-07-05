# Handling net messages

The `net_messages` example prints a few user messages from a demo. Run it with:

```bash
cargo run --example net_messages -- -demo path/to/demo.dem
```

It registers handlers using `Parser::register_net_message_handler` which now supports all message types defined in the legacy Go `net_messages.go`.
