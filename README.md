<div align="center">

<h1>LARGO</h1>

<p>Drop it. LARGO handles the rest.</p>

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Docker](https://img.shields.io/badge/docker-compose-2496ED?logo=docker&logoColor=white)](./docker-compose.yml)
[![Rust](https://img.shields.io/badge/rust-gateway-CE422B?logo=rust&logoColor=white)](./gateway)
[![Python](https://img.shields.io/badge/python-ai--worker-3776AB?logo=python&logoColor=white)](./ai-worker)
[![React](https://img.shields.io/badge/react-dashboard-61DAFB?logo=react&logoColor=white)](./web)

</div>

---

LARGO is a self-hosted personal finance and task management system. Photograph a receipt, type a transaction, create a task — the AI pipeline structures everything and surfaces it in a clean dashboard. No cloud storage. No subscriptions. Your data stays on your machine.

## Features

- **Receipt scanning** — photograph any NF-e; Gemini extracts total, items, CNPJ and date automatically
- **Natural language input** — type expenses and tasks in plain text; the AI structures them for you
- **Financial dashboard** — spending by category, monthly evolution, and custom date ranges
- **Task management** — create, track and close tasks from the same interface
- **Real-time feedback** — WebSocket notifications when AI processing completes
- **Privacy-first** — all data lives locally; only AI inference calls reach external APIs

## Architecture

LARGO follows a **Microservices Lite** pattern orchestrated by Docker Compose, with an internal network isolating all inter-service communication.

```
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

| Service | Technology | Role |
|---|---|---|
| `gateway` | Rust / Axum | Single entry point — auth (JWT), routing, orchestration, WebSocket hub |
| `ai-worker` | Python / FastAPI | TOON-encoded prompts → Gemini → structured JSON |
| `web` | React + Bun + Vite, Tailwind + shadcn/ui | Dashboard, data visualization, real-time updates |
| `mongo` | MongoDB 6 | Persistence — `expenses`, `tasks`, `users` collections |

### Internal design patterns

- **Backend** — Hexagonal Architecture (Ports & Adapters): domain logic is fully decoupled from infrastructure (`GeminiAdapter`, `MongoAdapter`, `AxumRouter`).
- **Frontend** — Feature-Based Architecture: code is organized by business module (`Finance`, `Tasks`) with presentational components and custom hooks that isolate all API/state logic.
- **AI prompts** — [TOON (Token-Oriented Object Notation)](./docs/ai/prompts.md): structured context sent to Gemini uses a compact token-efficient encoding (~40% fewer tokens vs raw JSON).

### AI pipeline (NF-e)

```
Photo → gateway → ai-worker → Gemini Flash (image OCR)
                             → TOON-encoded context
                             → Gemini Flash Lite (structuring)
                             → { total, itens[], data, cnpj, estabelecimento }
                             → MongoDB ← gateway ← WebSocket notification → web
```

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- A [Google AI Studio](https://aistudio.google.com) API key (Gemini)

### Installation

```sh
git clone https://github.com/seuusuario/largo.git
cd largo
cp .env.example .env
# fill in your credentials
docker compose up --build
```

Open `http://localhost:5173`.

### Environment Variables

| Variable | Description |
|---|---|
| `GEMINI_API_KEY` | Google AI Studio key |
| `GEMINI_MODEL_IMAGE` | Gemini model for image/OCR — default: `gemini-3.1-flash` |
| `GEMINI_MODEL_TEXT` | Gemini model for text — default: `gemini-3.1-flash-lite` |
| `JWT_SECRET` | Secret for signing JWT tokens |
| `MONGO_INITDB_ROOT_USERNAME` | MongoDB root username |
| `MONGO_INITDB_ROOT_PASSWORD` | MongoDB root password |
| `MONGO_DB_NAME` | Database name |

See [`.env.example`](./.env.example) for the full list.

## Project Structure

```
largo/
├── gateway/           # Rust/Axum — API gateway (Hexagonal Architecture)
├── ai-worker/         # Python/FastAPI — Gemini OCR + TOON prompt pipeline
├── web/               # React/Bun/Vite — dashboard (Feature-Based Architecture)
├── docs/              # ADRs, TOON prompt schemas, database specs
├── docker-compose.yml
└── .env.example
```

Key docs:

- [`docs/architecture/adr/`](./docs/architecture/adr/) — architectural decision records
- [`docs/ai/prompts.md`](./docs/ai/prompts.md) — TOON prompt schemas and examples
- [`docs/database/setup.md`](./docs/database/setup.md) — index and validator setup

## Contributing

Contributions are welcome. Before submitting a pull request for significant changes, please open an issue to discuss scope.

- Record architectural decisions as ADRs in `docs/architecture/adr/`.
- Update [`docs/ai/prompts.md`](./docs/ai/prompts.md) whenever a Gemini prompt schema changes.
- Keep `.env.example` in sync with any new environment variables.

## License

[MIT](./LICENSE)
