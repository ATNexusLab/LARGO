<div align="center">

<h1>LARGO</h1>

<p>Drop it. LARGO handles the rest.</p>

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Status](https://img.shields.io/badge/status-bootstrap-yellow)](./.github/tasks/todo.md)
[![Docker](https://img.shields.io/badge/docker--compose-mongo--only-2496ED?logo=docker&logoColor=white)](./docker-compose.yml)
[![Rust](https://img.shields.io/badge/rust-gateway--foundation-CE422B?logo=rust&logoColor=white)](./docs/backend/README.md)
[![Python](https://img.shields.io/badge/python-ai--worker--target-3776AB?logo=python&logoColor=white)](./docs/ai/README.md)
[![React](https://img.shields.io/badge/react-frontend--target-61DAFB?logo=react&logoColor=white)](./docs/frontend/README.md)

</div>

---

LARGO is a self-hosted personal finance and task management system. Photograph a receipt, type a transaction, create a task — the AI pipeline structures everything and surfaces it in a clean dashboard. No cloud storage. No subscriptions. Your data stays on your machine.

## Current Repository Status

This repository is currently in a **recovery/bootstrap phase** on branch `feat/recovery-foundation-start`.

- The **product vision and target architecture** are still documented in this README and in `docs/`.
- A **versioned Rust gateway foundation** now exists in [`gateway/`](./gateway), including `GET /healthz`, `POST /tasks`, Mongo connectivity validation, and the `db-init` smoke path binary.
- The **only component runnable via `docker compose` today** is MongoDB in [`docker-compose.yml`](./docker-compose.yml).
- `ai-worker/` and `web/` are still not versioned in this branch.
- The approved recovery sequence is tracked in [`.github/tasks/todo.md`](./.github/tasks/todo.md).

## Product Vision

The items below describe the **intended product capabilities**, not features that are already runnable from this branch today.

- **Receipt scanning** — photograph any NF-e; Gemini extracts total, items, CNPJ and date automatically
- **Natural language input** — type expenses and tasks in plain text; the AI structures them for you
- **Financial dashboard** — spending by category, monthly evolution, and custom date ranges
- **Task management** — create, track and close tasks from the same interface
- **Real-time feedback** — WebSocket notifications when AI processing completes
- **Privacy-first** — all data lives locally; only AI inference calls reach external APIs

## Architecture

### Target architecture

LARGO is intended to follow a **Microservices Lite** pattern orchestrated by Docker Compose, with an internal network isolating all inter-service communication.

```text
┌─────────────────────────────────────────────┐
│                   localhost                 │
│                                             │
│  ┌───────────┐          ┌────────────────┐  │
│  │    web    │◄────────►│    gateway     │  │
│  │ React/Bun │  REST +  │   Rust/Axum    │  │
│  └───────────┘    WS    └───────┬────────┘  │
│                                 │           │
│                       ┌─────────┴────────┐  │
│                       │                  │  │
│               ┌───────▼──────┐  ┌────────▼─┤│
│               │  ai-worker   │  │  MongoDB ││
│               │  Python/     │  │          ││
│               │  FastAPI     │  └──────────┘│
│               └──────┬───────┘              │
└──────────────────────┼──────────────────────┘
                       │
              ┌────────▼────────┐
              │   Google AI     │
              │  Gemini Flash   │  ← images (NF-e OCR)
              │  Gemini Flash   │
              │      Lite       │  ← text (natural language)
              └─────────────────┘
```

| Service | Technology | Role | Status in this branch |
|---|---|---|---|
| `gateway` | Rust / Axum | Single entry point — auth (JWT), routing, orchestration, WebSocket hub | Foundation versioned in repo; not started by compose |
| `ai-worker` | Python / FastAPI | TOON-encoded prompts → Gemini → structured JSON | Planned |
| `web` | React + Bun + Vite, Tailwind + shadcn/ui | Dashboard, data visualization, real-time updates | Planned |
| `mongo` | MongoDB 6 | Persistence — `expenses`, `tasks`, `users` collections | Available via `docker compose` |

### Internal design patterns

- **Backend** — Hexagonal Architecture (Ports & Adapters): domain logic is fully decoupled from infrastructure (`GeminiAdapter`, `MongoAdapter`, `AxumRouter`).
- **Frontend** — Feature-Based Architecture: code is organized by business module (`Finance`, `Tasks`) with presentational components and custom hooks that isolate all API/state logic.
- **AI prompts** — [TOON (Token-Oriented Object Notation)](./docs/ai/prompts.md): structured context sent to Gemini uses a compact token-efficient encoding (~40% fewer tokens vs raw JSON).

### Target AI pipeline (NF-e)

```text
Photo → gateway → ai-worker → Gemini Flash (image OCR)
                             → TOON-encoded context
                             → Gemini Flash Lite (structuring)
                             → { total, itens[], data, cnpj, estabelecimento }
                             → MongoDB ← gateway ← WebSocket notification → web
```

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose

> A Google AI key will only be needed after the planned `ai-worker` becomes part of the repository.

### Bootstrap the current local dependency

```sh
git clone https://github.com/ATNexusLab/LARGO.git
cd largo
cp .env.example .env
docker compose up -d mongo
```

This starts **only MongoDB** on `localhost:27017`.

What you should expect today:

- `docker compose ps` shows the `mongo` container
- there is **no** versioned web app to open at `localhost:5173`
- the compose file does **not** start the versioned gateway foundation yet
- there is **no** versioned AI worker started by the compose file yet

### Gateway foundation: manual path today

The repository already contains the first versioned backend foundation in [`gateway/`](./gateway), but it is **not wired into `docker-compose.yml` as a running service yet**.

Current implemented foundation:

- `GET /healthz` returns `200 OK`
- `POST /tasks` validates input and persists to MongoDB
- Mongo connectivity validation exists in code
- `db-init` exists as a Rust binary for the initial smoke path

For host-side Rust commands, use a host-reachable Mongo URI (the example in [`.env.example`](./.env.example) uses the Docker network hostname `mongo` for future container-to-container calls):

```sh
export MONGO_URI="mongodb://admin:change_me@127.0.0.1:27017/largo?authSource=admin"
export MONGO_DB_NAME=largo
cargo run -p gateway --bin db-init
```

Useful verification path today:

```sh
docker compose up -d mongo
cargo test -p gateway
```

### Environment Variables

#### Used by the current `docker-compose.yml`

| Variable | Description |
|---|---|
| `MONGO_INITDB_ROOT_USERNAME` | MongoDB root username |
| `MONGO_INITDB_ROOT_PASSWORD` | MongoDB root password |
| `MONGO_DB_NAME` | Database name |

#### Used by the versioned gateway foundation

| Variable | Description |
|---|---|
| `MONGO_URI` | MongoDB connection string used by host-side/manual Rust commands today |
| `MONGO_DB_NAME` | Database selected by `db-init` and gateway foundation code |

#### Already defined for planned next services

| Variable | Intended consumer |
|---|---|
| `JWT_SECRET` | Planned `gateway` auth layer (not used by the current foundation flow) |
| `GOOGLE_API_KEY` | Planned `ai-worker` Gemini integration |
| `GEMINI_MODEL_IMAGE` | Planned image/OCR model |
| `GEMINI_MODEL_TEXT` | Planned text model |
| `VITE_API_BASE_URL` | Planned frontend API base URL |

See [`.env.example`](./.env.example) for the full list and placeholder values.

## Project Structure

```text
largo/
├── gateway/                 # Versioned Rust gateway foundation + db-init binary
├── docs/                    # Product vision, ADRs, backend/frontend/AI docs
├── .github/tasks/todo.md    # Recovery/bootstrap backlog
├── docker-compose.yml       # Current bootstrap: MongoDB only
├── .env.example             # Environment placeholders for current + planned services
└── README.md
```

Runtime directories not yet versioned in this branch: `ai-worker/`, `web/`.

Key docs:

- [`docs/index.md`](./docs/index.md) — documentation hub with current-vs-target framing
- [`docs/architecture/adr/`](./docs/architecture/adr/) — architectural decision records
- [`docs/ai/prompts.md`](./docs/ai/prompts.md) — TOON prompt schemas and examples
- [`docs/database/setup.md`](./docs/database/setup.md) — index and validator setup

## Contributing

Contributions are welcome. Before submitting a pull request for significant changes, please open an issue to discuss scope.

- Record architectural decisions as ADRs in `docs/architecture/adr/`.
- Update [`docs/ai/prompts.md`](./docs/ai/prompts.md) whenever a Gemini prompt schema changes.
- Keep `.env.example` in sync with any new environment variables.
- When new services land, update docs to move items from **planned** to **implemented**.

## License

[MIT](./LICENSE)
