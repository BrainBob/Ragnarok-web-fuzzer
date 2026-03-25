⚔ Ragnarök
<div align="center">
    ██████╗  █████╗  ██████╗ ███╗   ██╗ █████╗ ██████╗  ██████╗ ██╗  ██╗
    ██╔══██╗██╔══██╗██╔════╝ ████╗  ██║██╔══██╗██╔══██╗██╔═══██╗██║ ██╔╝
    ██████╔╝███████║██║  ███╗██╔██╗ ██║███████║██████╔╝██║   ██║█████╔╝
    ██╔══██╗██╔══██║██║   ██║██║╚██╗██║██╔══██║██╔══██╗██║   ██║██╔═██╗
    ██║  ██║██║  ██║╚██████╔╝██║ ╚████║██║  ██║██║  ██║╚██████╔╝██║  ██╗
    ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝

Born in Asgard. Forged in Battle.
Fast, recursive, async web fuzzing written in Rust.


**Born in Asgard. Forged in Battle.**

Fast, recursive, async web fuzzing written in Rust.

</div>

---

## 🚀 Overview

**Ragnarök** is a high-performance web content discovery tool designed for:

- Penetration testers
- Red team operators
- Bug bounty hunters
- CTF players

It brute-forces endpoints using wordlists, detects valid resources, and recursively expands attack surface — all with async concurrency.

---

## ⚙️ Key Features

- ⚡ **Blazing Fast** — async runtime powered by `tokio`
- 🌿 **Recursive Discovery** — automatic traversal of discovered directories
- 🔮 **Extension Fuzzing** — append `.php`, `.html`, `.bak`, etc.
- 🌊 **Wildcard Detection** — eliminate false positives
- 🔀 **Proxy Support** — HTTP / SOCKS5 (Burp compatible)
- 🔁 **Replay Proxy** — re-send hits to another proxy
- 🛡 **Custom Headers & Auth** — full header + cookie control
- 📬 **All HTTP Methods** — GET, POST, PUT, DELETE
- ⚖ **Rate Limiting** — control request flow
- 🔍 **Advanced Filtering** — by status, size, words
- 💾 **Structured Output** — TXT / JSON export
- 🎨 **Clean CLI Output** — color-coded + progress indicators

---

## 🧠 Architecture

| Component | Description |
|----------|------------|
| Tokio | Async runtime |
| Reqwest | HTTP client |
| DashMap | Lock-free concurrent storage |
| Futures | Concurrency orchestration |

---

## 📦 Installation

### Requirements

- Rust **1.75+**

### Build from Source

```bash
git clone https://github.com/YOUR_USERNAME/ragnarok
cd ragnarok
cargo build --release

sudo cp target/release/ragnarok /usr/local/bin/
```

Pre-built Binaries
Download from Releases:
Linux (x86_64)
Windows (x86_64)
macOS (Intel / ARM)

⚡ Quick Start

```
# Basic scan
ragnarok -u http://target.com -w wordlist.txt

# With extensions
ragnarok -u http://target.com -w wordlist.txt \
  --runes php,html,txt

# Recursive scan
ragnarok -u http://target.com -w wordlist.txt \
  --recurse -d 2

# Through Burp
ragnarok -u http://target.com -w wordlist.txt \
  -x http://127.0.0.1:8080 \
  --replay-proxy http://127.0.0.1:8080 \
  --json results.json
```

🧬 Fuzzing Marker (ODIN)
Ragnarök uses ODIN as a substitution marker (similar to FUZZ in ffuf).

```aiignore
# Path fuzzing
ragnarok -u http://target.com/ODIN -w dirs.txt

# Query fuzzing
ragnarok -u "http://target.com/search?q=ODIN" -w payloads.txt

# POST fuzzing
ragnarok -u http://target.com/login -w passwords.txt \
  -X POST --body "username=admin&password=ODIN"

# Subdomain fuzzing
ragnarok -u http://ODIN.target.com -w subs.txt
```

🛠 Usage
Core Flags

| Flag             | Description               |
| ---------------- | ------------------------- |
| `-u, --url`      | Target URL                |
| `-w, --wordlist` | Wordlist file             |
| `-W, --warriors` | Concurrency level         |
| `-T, --timeout`  | Request timeout           |
| `--recurse`      | Enable recursion          |
| `-d, --depth`    | Max recursion depth       |
| `--runes`        | File extensions           |
| `--wildcard`     | Enable wildcard detection |

Filtering

| Flag             | Description           |
| ---------------- | --------------------- |
| `--min-size`     | Minimum response size |
| `--max-size`     | Maximum response size |
| `--filter-words` | Filter by word count  |
| `--valhalla`     | Allowed status codes  |

Networking

| Flag             | Description         |
| ---------------- | ------------------- |
| `-x, --proxy`    | Proxy (HTTP/SOCKS5) |
| `--replay-proxy` | Replay hits         |
| `--rate-limit`   | Requests per second |

Request Control

| Flag            | Description    |
| --------------- | -------------- |
| `-X, --method`  | HTTP method    |
| `--body`        | Request body   |
| `-H, --headers` | Custom headers |
| `-b, --cookies` | Cookies        |

Output

| Flag         | Description         |
| ------------ | ------------------- |
| `-l, --loot` | Save results (TXT)  |
| `--json`     | Save results (JSON) |


📊 Output Format

```aiignore
🌍 [ROOT]    [200]  ⚔  SLAIN      11329b    0w  http://target.com/ftp
🌿 [DEPTH 1] [403]  🪓 FORBIDDEN      0b    0w  http://target.com/private
```

| Status  | Label        | Meaning            |
| ------- | ------------ | ------------------ |
| 200     | ⚔ SLAIN      | Accessible         |
| 204     | 🛡 SHIELD    | No content         |
| 301/302 | 🌊 VOYAGE    | Redirect           |
| 401     | 🔒 RUNE LOCK | Unauthorized       |
| 403     | 🪓 FORBIDDEN | Forbidden          |
| 405     | ⛩ SEALED     | Method not allowed |

🔥 Examples

```aiignore
# HTB / lab
ragnarok -u http://10.10.10.10 \
  -w /usr/share/wordlists/dirb/common.txt \
  -W 100 --valhalla 200,301,403

# API fuzzing
ragnarok -u http://target.com/api/v1/ODIN \
  -w api.txt --valhalla 200,401

# Authenticated scan
ragnarok -u http://target.com \
  -w wordlist.txt \
  -H "Authorization: Bearer TOKEN" \
  -b "session=abc123"

# Aggressive scan
ragnarok -u http://target.com \
  -w raft-large.txt \
  -W 150 --rate-limit 50 \
  --recurse -d 3 \
  --wildcard \
  --json results.json
```

🧭 Terminology

| Term      | Meaning             |
| --------- | ------------------- |
| ODIN      | Fuzz marker         |
| Warriors  | Threads             |
| Runes     | Extensions          |
| Valhalla  | Status filter       |
| Shield    | User-Agent          |
| Loot      | Output              |
| Yggdrasil | Recursive traversal |

⚖️ Legal Disclaimer
Ragnarök is intended for authorized security testing only.
Do not use against systems without explicit permission.
The author is not responsible for misuse or damage.
<div align="center">
Odin watches from Hlidskjalf — and sees every path 🐦‍⬛
</div> ```