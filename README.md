# dockode_bot

Telegram bot for dockode server. Responds to commands with system metrics and service status.

## Commands

| Command | Description |
|---------|-------------|
| `/stats` | Full server metrics (CPU, memory, disk, uptime, tunnel) |
| `/cpu` | CPU usage and load average |
| `/mem` | RAM and swap usage |
| `/disk` | Disk usage for all mounts |
| `/uptime` | System uptime and boot time |
| `/tunnel` | VSCode Tunnel status |

## Architecture

- **Framework**: teloxide v0.17 with `Command::repl` for command dispatching
- **Runtime**: tokio async runtime with `rt-multi-thread`
- **Transport**: rustls (TLS) via reqwest — no OpenSSL dependency
- **Polling**: Long polling via `UpdateListener` (getUpdates loop)
- **Network**: `network_mode: host` required for IPv6 connectivity to Telegram API

## Build

### Prerequisites

- Rust 1.70+ (for `cargo`)
- `musl-tools` (for cross-compilation to musl target)

### Local development

```bash
# Standard build (GLIBC)
cargo build --release

# Static musl build (portable, recommended)
cargo build --release --target x86_64-unknown-linux-musl
```

### Why musl + rustls?

The bot runs inside an Ubuntu 22.04 container. To avoid GLIBC version mismatches and eliminate OpenSSL build dependencies:

- `default-features = false` removes `native-tls` from teloxide
- `features = ["rustls"]` uses rustls instead of OpenSSL
- `x86_64-unknown-linux-musl` target produces a fully static binary

## Release Pipeline

Releases are automated via GitHub Actions:

1. Push a version tag: `git tag v0.3.0 && git push origin v0.3.0`
2. CI builds the musl binary
3. Binary and SHA256 checksum are attached to the GitHub Release

**Download URL** (latest release):
```
https://github.com/mehfius/dockode_bot/releases/latest/download/telegram-bot
```

## Docker Integration

The Dockerfile downloads the pre-built binary from GitHub Releases:

```dockerfile
RUN curl -L https://github.com/mehfius/dockode_bot/releases/latest/download/telegram-bot \
    -o /usr/local/bin/telegram-bot && chmod +x /usr/local/bin/telegram-bot
```

To rebuild the Docker image:

```bash
cd /home/m/mehfius/docker/dockode
docker compose up --build
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `TELEGRAM_TOKEN` | Bot API token from @BotFather |
| `CHAT_ID` | Telegram chat ID to send messages to |

Set in `docker-compose.yml` or environment.

## Version History

- **v0.3.0**: Switch to rustls + musl static binary, GitHub Actions release pipeline
- **v0.2.0**: teloxide rewrite with command handlers
- **v0.1.0**: Initial bot with basic messaging

## License

MIT