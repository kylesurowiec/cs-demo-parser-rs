# Entity Population Tasks

This file tracks progress on the items listed in `entity_population_checklist.md`.
Check off tasks as implementations are completed.

- [x] Parse server classes from `svc_ServerInfo` and datatable messages so entity definitions are available before frames are processed.
- [x] Decode string tables, especially `userinfo` and item definition tables, to obtain player names and equipment mapping.
- [x] Populate `GameState.players_by_user_id` and `players_by_entity_id` whenever `player_connect` or entity creation events are seen.
- [x] Update `Player` fields such as `steam_id64`, `name`, `team`, and `entity_id` from the parsed data.
- [x] Track equipment entities and fill `GameState.weapons` with the correct `Equipment` objects.
- [x] Record grenade projectiles and owners inside `GameState.grenade_projectiles` and `projectile_owners`.
- [x] Maintain the `Bomb` state, updating its carrier and last ground position based on entity updates and game events.
- [x] Keep `Hostage` and `Inferno` instances in sync with their entity data.
- [x] Insert every received `Entity` into `GameState.entities` so later lookups can access raw properties.
- [x] Update team scores and match information during round and game‑phase events.
