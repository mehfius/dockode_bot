# dockode_bot

Telegram bot for dockode server notifications.

Built in Rust, sends startup and status messages to a Telegram chat via the Bot API.

## How it works

The bot reads two environment variables, takes a message as argument, and sends it via the Telegram API:

```bash
TELEGRAM_TOKEN=xxx CHAT_ID=123 telegram-bot "Servidor ligou"
```

## Usage

```bash
TELEGRAM_TOKEN=<your_bot_token> CHAT_ID=<your_chat_id> ./telegram-bot "Hello World"
```

## Build

```bash
cargo build --release
```

Binary: `target/release/telegram-bot`

## Release

Tag a version to trigger the GitHub Action:

```bash
git tag v0.1.0 && git push --tags
```

The action compiles and publishes the binary to the release page.

## License

MIT
