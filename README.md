# 🚀 Rayconf

CLI configuration manager for [XRay-core](https://github.com/xtls/xray-core).

[![Rust](https://github.com/pokatomnik/rayconf/actions/workflows/rust.yml/badge.svg)](https://github.com/pokatomnik/rayconf/actions/workflows/rust.yml)

## ℹ️ What is it

Rayconf converts XRay outbound URLs (`vless://`, `vmess://`, `trojan://`, `ss://`, `socks://`) into valid JSON configuration files that XRay can consume directly. It also fetches and decodes subscription (remote) links — lists of servers shared in plain-text or Base64-encoded format — so you can pick the best server without writing a single line of JSON by hand.

Output goes to stdout, meant to be piped straight into `xray run -c stdin:` (or `xray run` without `-c` — XRay reads from stdin by default when no config file is given).

## 🤔 Why

GUI apps like [Happ](https://github.com/Happ-proxy/happ-desktop) and [v2raytun](https://github.com/mdf45/v2raytun/) are great, but if you prefer the original `xray` CLI, you need to manage configurations yourself. Subscription links (e.g. `www.example.com/subscription/abcde`) and outbound URLs (e.g. `vless://...`) must be translated into verbose XRay JSON. Rayconf does that translation for you, interactively, right in the terminal.

## 📡 Supported protocols

| Protocol    | URI scheme                             |
| ----------- | -------------------------------------- |
| VLESS       | `vless://`                             |
| VMess       | `vmess://`                             |
| Trojan      | `trojan://`                            |
| Shadowsocks | `ss://`                                |
| SOCKS       | `socks://` / `socks4://` / `socks5://` |

Supported transports: TCP, WebSocket, gRPC, QUIC, KCP, XHTTP.  
Supported security layers: TLS, Reality.

## 🛠️ Tech stack

- **Rust** (edition 2024)
- **tokio** — async runtime
- **clap** — CLI argument parsing with derive macros
- **dialoguer** — interactive fuzzy-select prompts, multi-select, input
- **reqwest** — HTTP client for fetching subscription URLs
- **serde / serde_json** — configuration serialization
- **base64** — decoding Base64-encoded subscription lists
- **dnsclient** — DNS resolution for server reachability checks
- **futures** — concurrent TCP probes (buffered, unordered)
- **spinners** — terminal spinners during long operations

## 📦 Installation

### 🏗️ Build from source

```shell
git clone https://github.com/pokatomnik/rayconf.git
cd rayconf
cargo build --release
```

The binary will be at `target/release/rayconf`. Move it to a directory in your `$PATH`.

### 📥 Download from GitHub Releases

Pre-built binaries are available on the [Releases](https://github.com/pokatomnik/rayconf/releases) page. Download the archive for your platform, extract, and place the binary in your `$PATH`.

## 🖥️ Shell completion

Rayconf can generate completion scripts for Bash, Zsh, Fish, and other shells:

```shell
rayconf completion --shell bash  > /usr/share/bash-completion/completions/rayconf
rayconf completion --shell zsh   > /usr/share/zsh/site-functions/_rayconf
rayconf completion --shell fish  > ~/.config/fish/completions/rayconf.fish
```

## ⚙️ Usage

### ➕ Add a local server

Store an outbound URL so you can reuse it later:

```shell
rayconf add vless://uuid@server:443?security=reality&sni=example.com&fp=chrome&type=tcp&flow=xtls-rprx-vision#MyServer
```

Or run `rayconf add` without arguments — you will be prompted interactively.

Alias: `rayconf a`

### 🗑️ List and remove local servers

```shell
rayconf delete
```

Select one or more servers from the list to remove. Aliases: `rayconf d`, `rayconf r`, `rayconf remove`

### 🔗 Connect to a local server

```shell
rayconf select | xray run
```

You will be prompted to pick a server from saved ones. The generated JSON config is printed to stdout and piped into XRay.

### 🌐 Manage subscription (remote) URLs

```shell
rayconf remote add     # add a new subscription
rayconf remote remove  # remove a subscription
rayconf remote list    # list all subscriptions
```

When adding a remote, you specify:

- **alias** — a name to identify the subscription
- **URL** — the subscription endpoint
- **decoder** — `plain` (newline-separated URLs) or `base64` (Base64-encoded blob)
- **custom HTTP headers** (optional) — add arbitrary headers to the subscription request

### 📡 Connect to a server from a subscription

```shell
rayconf select --remote | xray run
```

Rayconf fetches the subscription, decodes the server list, probes each server over TCP (measures round-trip time), and shows servers sorted by response time with dead/unreachable servers marked. Pick one and the JSON config is piped to XRay.

### 🎛️ Select options

| Flag             | Description                                                      |
| ---------------- | ---------------------------------------------------------------- |
| `--remote`, `-r` | Use servers from subscription URLs instead of locally saved ones |
| `--socks-port`   | Override the default SOCKS5 inbound port (default: `1080`)       |
| `--http-port`    | Use an HTTP inbound instead of SOCKS5 (default: `8080`)          |
| `--log-level`    | Set XRay log level: `none`, `debug`, `info`, `warning`, `error`  |
| `--log-dns`      | Enable DNS query logging in XRay                                 |
| `--skip-check`   | Skip the TCP reachability probe; show servers immediately        |

> `--socks-port` and `--http-port` are mutually exclusive. Without either, the config defaults to a SOCKS5 inbound on port `1080`.

## 💾 Configuration storage

Server URLs and subscription metadata are stored in `~/.config/rayconf/rayconf.json`. The file is auto-created on first use.

## 📊 How server sorting works

When you run `rayconf select --remote`, Rayconf:

1. Fetches the subscription URL.
2. Decodes the server list (plain or Base64).
3. Parses each URL into an XRay server entry.
4. Runs concurrent TCP probes (up to 5 in parallel) to measure round-trip time.
5. Sorts servers from fastest to slowest; servers that do not respond within 10 seconds are marked as dead.
6. Displays the sorted list with per-server latency.

Use `--skip-check` to bypass the probe and show the raw server list immediately.

## 🙏 Acknowledgements

- [XRay-core](https://github.com/xtls/xray-core) — the engine that keeps the internet open
- [AvenCores](https://github.com/AvenCores/goida-vpn-configs) — free VPN configurations (Гойда!)
- [Keivan-sf](https://github.com/Keivan-sf/v2-uri-parser) — original URI parsing logic that powers the config generator

## 👥 Authors

- [@pokatomnik](https://github.com/pokatomnik) — project author and maintainer
- [Keivan-sf](https://github.com/Keivan-sf/v2-uri-parser) — author of the original idea

## 📄 License

[BSD Zero Clause License](LICENSE) — do whatever you want with this code.
