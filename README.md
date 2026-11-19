# SigmaOS Snapshot Manager

BTRFS snapshot lifecycle management with Command, Factory, and Singleton patterns.

## OOPS Principles

- **Command Pattern**: Snapshot operations as commands (create, delete)
- **Factory Pattern**: Create different snapshot types (pre-update, manual)
- **Singleton Pattern**: Single snapshot manager instance

## Features

- Pre-update snapshots
- Manual snapshots
- Automatic cleanup (retention policy)
- Snapshot boot selection

## Usage

```bash
sigmaos-snapshot-manager pre-update "Before kernel update"
sigmaos-snapshot-manager manual "Manual backup"
```

## Testing

```bash
cargo test --offline
```
