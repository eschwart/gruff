# Gruff

Gruff is a lightweight Discord bot written in Rust using the [serenity](https://crates.io/crates/serenity) library. It automatically assigns predefined roles to new members when they join a server.

## Features

- Auto-assign roles to new members
- Configurable per guild via `config.json`
- Fast, efficient, and minimal

## Setup

1. Build and run the bot:

```bash
cargo run --release -- --token YOUR_BOT_TOKEN --config CONFIG_TXT
```

## Permissions
| Type        | Required                        |
|-------------|---------------------------------|
| Scope(s)    | `bot`                           |
| Permissions | `Manage Roles`, `View Channels` |
