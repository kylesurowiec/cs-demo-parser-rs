# Capturing Voice Data

This example extracts the in‑game voice chat from a demo and saves the raw audio
payloads to disk. Run it from the repository root:

```bash
cargo run --example voice_capture -- -demo <path/to/demo.dem> -out voice.raw
```

The written file contains the encoded voice stream. Converting the audio to a
playable format depends on the game and may require external tools. More details
can be found in the following projects:

- CS2: <https://github.com/DandrewsDev/CS2VoiceData>
- CS:GO: <https://github.com/saiko-tech/csgo-demo-voice-capture-example>
