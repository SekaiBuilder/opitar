# CAM-003 — Plan

1. Rename spec files and translate all specs, README, NOTICE, CLAUDE.md/AGENTS.md.
2. `kind.rs`: English kind names. `scan.rs`: English field names, cache v2.
3. `store.rs`: v2 structs with serde aliases for v1, state mapping, fixture test for SEL-01-A04.
4. `server.rs`, `main.rs`: English API keys, messages and flags with hidden Spanish aliases.
5. `assets/index.html`: English UI and filters.
6. `cargo test`, `cargo clippy`, browser walk-through, commit, push.
