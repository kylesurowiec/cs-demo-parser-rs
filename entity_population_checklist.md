# Entity Population Checklist

Ensure that each game entity is correctly created and updated while parsing demos. Use this checklist when expanding the parser to cover additional data tables or events.

- [ ] Parse server classes from `svc_ServerInfo` and datatable messages
      so entity definitions are available before frames are processed.
- [ ] Decode string tables, especially `userinfo` and item definition tables,
      to obtain player names and equipment mapping.
- [ ] Populate `GameState.players_by_user_id` and `players_by_entity_id`
      whenever `player_connect` or entity creation events are seen.
- [ ] Update `Player` fields such as `steam_id64`, `name`, `team`, and
      `entity_id` from the parsed data.
- [ ] Track equipment entities and fill `GameState.weapons` with the correct
      `Equipment` objects.
- [ ] Record grenade projectiles and owners inside
      `GameState.grenade_projectiles` and `projectile_owners`.
- [ ] Maintain the `Bomb` state, updating its carrier and last ground position
      based on entity updates and game events.
- [ ] Keep `Hostage` and `Inferno` instances in sync with their entity data.
- [ ] Insert every received `Entity` into `GameState.entities` so later lookups
      can access raw properties.
- [ ] Update team scores and match information during round and
      game‑phase events.

Check off items as implementations are completed to track progress.
