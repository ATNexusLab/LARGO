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

LARGO is a self-hosted personal life management system. Photograph a receipt, type a task, drop a note — LARGO organizes everything through an AI-powered pipeline and surfaces it in a clean financial dashboard. No cloud. No subscriptions. Your data stays on your machine.

## Features

- **Receipt scanning** — photograph any NF-e and LARGO extracts total, items, CNPJ and date automatically
- **Natural language input** — type expenses and tasks in plain text; the AI structures them for you
- **Financial dashboard** — spending by category, monthly evolution, and custom date ranges
- **Task management** — create, track and close tasks from the same interface
- **Privacy first** — all data lives locally; only OCR and AI inference calls reach external APIs

## Architecture

LARGO runs as four isolated services orchestrated by Docker Compose.

```
┌────────────────────────────────────────────┐
│                  localhost                 │
│                                            │
│   ┌──────────┐         ┌────────────────┐  │
│   │   web    │◄───────►│    gateway     │  │
│   │  React   │  REST   │   Rust/Axum    │  │
│   └──────────┘         └───────┬────────┘  │~
│                                │           │
│                      ┌─────────┴────────┐  │
│                      │                  │  │
│               ┌──────▼──────┐  ┌────────▼─┐│
│               │  ai-worker  │  │  MongoDB ││
│               │   Python    │  │          ││
│               └──────┬──────┘  └──────────┘│
└──────────────────────┼─────────────────────┘
                       │
               ┌───────▼────────┐
               │   Google AI    │
               │  Vision API    │
               │  Gemini 3.1    │
               └────────────────┘
```

| Service | Technology | Role |
|---|---|---|
| `gateway` | Rust / Axum | Request routing, auth, orchestration |
| `ai-worker` | Python / FastAPI | OCR via Vision API + structuring via Gemini |
| `web` | React + Tailwind + Shadcn/ui | Dashboard and data visualization |
| `mongo` | MongoDB | Persistence for expenses and tasks |

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- A [Google Cloud](https://cloud.google.com/vision) account for Vision API
- A [Google AI Studio](https://aistudio.google.com) account for Gemini

### Installation

```sh
git clone https://github.com/seuusuario/largo.git
cd largo
cp .env.example .env
```

Edit `.env` with your credentials, then:

```sh
docker compose up --build
```

Open `http://localhost:5173`.

### Environment Variables

| Variable | Description |
|---|---|
| `GOOGLE_CLOUD_API_KEY` | Google Cloud Vision API key |
| `GEMINI_API_KEY` | Google AI Studio key (Gemini 3.1 Flash) |
| `JWT_SECRET` | Secret for signing JWT tokens |
| `MONGO_URI` | MongoDB connection string |

See [`.env.example`](./.env.example) for all variables.

## Project Structure

```
largo/
├── gateway/          # Rust/Axum — API gateway
├── ai-worker/        # Python/FastAPI — OCR + AI pipeline
├── web/              # React/Bun — dashboard
├── docs/             # Architecture decisions and references
├── docker-compose.yml
└── .env.example
```

## Contributing

Contributions are welcome. Please open an issue before submitting a pull request for significant changes.

## License

[MIT](./LICENSE)