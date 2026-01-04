# PAGI External API & LLM Provider Library (`pagi-external-api-lib`)

Centralized **external network I/O** + **LLM orchestration** for PAGI.

This crate is designed to keep “core logic” clean by placing these concerns in one library:

- Config / secret loading
- Outbound HTTP calls
- LLM provider orchestration (OpenRouter)

---

## What this crate does

- Loads configuration from environment variables (and an optional `.env` file)
- Calls **OpenRouter** Chat Completions (`/chat/completions`) asynchronously
- Provides placeholder external service clients (Jira, CrowdStrike) to show where real integrations will live

Non-goals (today):

- A complete Jira/CrowdStrike integration (clients are stubs/placeholders)
- Streaming chat (SSE)
- A stable, versioned public API guarantee (pre-1.0)

---

## Repository structure

```
.
├── Cargo.toml
├── README.md
└── src
    ├── api_clients.rs
    ├── config.rs
    ├── lib.rs
    └── llm_provider.rs
```

Key modules:
- `src/config.rs` — config + secret loading
- `src/llm_provider.rs` — OpenRouter LLM calls
- `src/api_clients.rs` — external service client placeholders
- `src/lib.rs` — re-exports for a clean public API

---

## Getting started

### 1) Install Rust

Install Rust using rustup:
- https://rustup.rs/

Verify:
```bash
rustc --version
cargo --version
```

### 2) Clone the repo

```bash
git clone git@github.com:c04ch1337/pagi-external-api-lib.git
cd pagi-external-api-lib
```

### 3) Configure environment variables

This crate loads variables from:
- your shell environment, and
- an optional `.env` file in the project root (useful for local dev)

Create `.env` (do **not** commit it):
```bash
cat > .env <<'EOF'
OPENROUTER_API_KEY=your_openrouter_key_here
OPENROUTER_DEFAULT_MODEL=openai/gpt-4o-mini

# Optional placeholders
JIRA_API_TOKEN=...
JIRA_BASE_URL=https://jira.example.com

CROWDSTRIKE_API_TOKEN=...
CROWDSTRIKE_BASE_URL=https://api.crowdstrike.com
EOF
```

#### Environment variable reference

| Variable | Required | Default | Used by |
|---|:---:|---|---|
| `OPENROUTER_API_KEY` | ✅ | (none) | [`PAGIConfig`](src/config.rs:8), [`LLMProvider`](src/llm_provider.rs:6) |
| `OPENROUTER_DEFAULT_MODEL` | ❌ | `openai/gpt-4o-mini` | [`LLMProvider`](src/llm_provider.rs:6) |
| `JIRA_API_TOKEN` | ❌ | empty string | [`JiraClient`](src/api_clients.rs:5) (placeholder) |
| `JIRA_BASE_URL` | ❌ | `https://jira.example.com` | [`JiraClient`](src/api_clients.rs:5) (placeholder) |
| `CROWDSTRIKE_API_TOKEN` | ❌ | empty string | [`CrowdstrikeClient`](src/api_clients.rs:33) (placeholder) |
| `CROWDSTRIKE_BASE_URL` | ❌ | `https://api.crowdstrike.com` | [`CrowdstrikeClient`](src/api_clients.rs:33) (placeholder) |

Required:

- `OPENROUTER_API_KEY`

Optional:

- `OPENROUTER_DEFAULT_MODEL` (defaults to `openai/gpt-4o-mini`)
- `JIRA_API_TOKEN`, `JIRA_BASE_URL`
- `CROWDSTRIKE_API_TOKEN`, `CROWDSTRIKE_BASE_URL`

Notes:

- Config loading is intentionally strict: [`PAGIConfig::load()`](src/config.rs:26) panics if `OPENROUTER_API_KEY` is missing.
- The placeholder clients ([`JiraClient`](src/api_clients.rs:5), [`CrowdstrikeClient`](src/api_clients.rs:33)) expect a full [`PAGIConfig`](src/config.rs:8), so `OPENROUTER_API_KEY` must be present even if you only intend to use the placeholders.

### 4) Basic verification (no code changes)

Run a quick build check:

```bash
cargo check
```

If you plan to run the OpenRouter example in the README below, confirm you have:

- a valid `OPENROUTER_API_KEY`
- outbound network access

---

## Build / run checks

```bash
cargo check
```

Optional but recommended:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

---

## Public API (quick overview)

Re-exported from [`src/lib.rs`](src/lib.rs:1):

- [`PAGIConfig`](src/config.rs:8) — loads config/secret values from `.env` + environment variables
- [`LLMProvider`](src/llm_provider.rs:6) — OpenRouter chat completions wrapper
- [`JiraClient`](src/api_clients.rs:5) — placeholder
- [`CrowdstrikeClient`](src/api_clients.rs:33) — placeholder

## Usage examples

### Example: Call OpenRouter to generate a response

```rust
use pagi_external_api_lib::LLMProvider;

#[tokio::main]
async fn main() {
    // `LLMProvider::new()` loads env config immediately; it will panic if
    // OPENROUTER_API_KEY is missing.
    let llm = LLMProvider::new();

    let text = llm
        .generate_response(
            "Write a 1-sentence summary of why config isolation matters.",
            "You are a helpful assistant.",
            None, // uses OPENROUTER_DEFAULT_MODEL
        )
        .await
        .expect("OpenRouter call failed");

    println!("LLM response: {text}");
}
```

#### Security note

Avoid logging request headers or full config objects that may include secrets.
If you add structured logging later, ensure `OPENROUTER_API_KEY` is always redacted.

### Example: Use placeholder Jira client

```rust
use pagi_external_api_lib::{JiraClient, PAGIConfig};

#[tokio::main]
async fn main() {
    let cfg = PAGIConfig::load();
    let jira = JiraClient::new(cfg);

    jira.create_issue("Investigate incident #123").await.unwrap();
}
```

### Example: Use as a dependency

If you want to use this crate from another repository/app, add it to your `Cargo.toml` via Git:

```toml
[dependencies]
pagi-external-api-lib = { git = "https://github.com/c04ch1337/pagi-external-api-lib.git", branch = "main" }
```

If you prefer the CLI:

```bash
cargo add pagi-external-api-lib --git https://github.com/c04ch1337/pagi-external-api-lib.git --branch main
```

---

## Architecture

### High-level view (crate responsibilities)

```mermaid
flowchart TB
  Core[🧠 PAGI Core Logic]
  Ext[🌐 pagi-external-api-lib]
  OR[🤖 OpenRouter API]
  Jira[📌 Jira API]
  CS[🛡️ CrowdStrike API]
  Env[🔐 .env + Environment Variables]

  Core -->|calls| Ext
  Env -->|loads| Ext
  Ext -->|HTTP| OR
  Ext -->|HTTP (future)| Jira
  Ext -->|HTTP (future)| CS
```

### Low-level view (LLM request flow)

```mermaid
sequenceDiagram
  autonumber
  participant App as App / Core
  participant LLM as LLMProvider
  participant HTTP as reqwest::Client
  participant OR as OpenRouter

  App->>LLM: generate_response(prompt, system_prompt, model?)
  LLM->>LLM: choose model (override or default_model)
  LLM->>HTTP: POST /chat/completions (Bearer OPENROUTER_API_KEY)
  HTTP->>OR: HTTPS request
  OR-->>HTTP: JSON response
  HTTP-->>LLM: parsed response
  LLM-->>App: choices[0].message.content (String)
```

### Configuration loading flow

```mermaid
flowchart LR
  A[Start] --> B[dotenvy::dotenv()]
  B --> C[Read env vars]
  C --> D{OPENROUTER_API_KEY set?}
  D -- No --> E[panic! (secure init)]
  D -- Yes --> F[PAGIConfig]
```

---

## Dependencies

From `Cargo.toml`:

| Dependency | Purpose |
|---|---|
| `tokio` | Async runtime |
| `reqwest` | HTTP client |
| `serde` + `serde_json` | Serialization / JSON |
| `dotenvy` | Loads `.env` files |
| `async-trait` | Async traits (for future provider abstractions) |

---

## Testing & debugging

### Testing

There are currently no dedicated unit/integration tests.

Recommended next steps:
- Add unit tests for config parsing (with controlled env vars)
- Add mocked HTTP tests for OpenRouter (e.g., using `wiremock`)

Run tests:
```bash
cargo test
```

### Debugging tips

Common issues:

1) **Missing env var**
- If `OPENROUTER_API_KEY` is missing, config loading will panic by design.

2) **HTTP errors**
- OpenRouter failures will return an error from `reqwest`.

Debug workflow:
- Confirm `.env` is present locally
- Print which model you’re sending
- Use `RUST_LOG` (when you add logging) to trace HTTP request/response metadata

Useful commands:
```bash
cargo check
cargo clippy
cargo fmt
```

---

## Contributing

Contributions are welcome—keep changes clean and reviewable.

### Guidelines

1) Create a feature branch:
```bash
git checkout -b feat/your-feature
```

2) Run formatting + linting:
```bash
cargo fmt
cargo clippy -- -D warnings
```

3) Keep commits small and descriptive

4) Open a PR with:
- what changed
- why it changed
- how it was tested

---

## Roadmap / future plans

Planned improvements (in rough priority order):

### LLM provider improvements
- Provider abstraction trait (OpenRouter, OpenAI, etc.)
- Better error types (avoid leaking secrets, include context)
- Retries with exponential backoff
- Timeouts, rate limiting, and circuit breaking
- Streaming responses (SSE) for chat UX

### Security improvements
- Secret redaction utilities
- Stronger config validation (explicit “required vs optional” schema)
- Support for external secret managers (Vault, AWS SSM, etc.)

### External API clients
- Real Jira REST client (issue create, status transitions, comments)
- Real CrowdStrike workflows (host lookup, containment actions)

### Testing
- Integration tests with mock servers
- Contract tests for payload formats

---

## License

License is currently **TBD**.
