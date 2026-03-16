# Prophetic Glorious Manifestation — Pastoral School Platform

Pastoral School for pastors, ministers and prophets (Owerri, Imo State), built as a monorepo with:

- A **SvelteKit frontend** for students and admins
- A **Rust (Axum) backend** for auth, registration, courses, payments and admin APIs
- A **Docusaurus docs site** for public/docs content

This README describes **this app**, not the original template, and links to the individual app READMEs.

---

## 📚 Table of Contents

1. [Overview](#overview)
2. [Monorepo Structure](#monorepo-structure)
3. [Apps & Services](#apps--services)
   - [Frontend (`apps/frontend`)](#frontend-appsfrontend)
   - [Backend (`apps/backend`)](#backend-appsbackend)
   - [Docs (`apps/docs`)](#docs-appsdocs)
   - [Shared & Infra](#shared--infra)
4. [Core Features](#core-features)
5. [Local Development](#local-development)
6. [Testing](#testing)
7. [Environments & Secrets](#environments--secrets)
8. [CI/CD](#cicd)
9. [Contributing](#contributing)
10. [License](#license)

---

## Overview

This repo powers the **Prophetic Glorious Manifestation Pastoral School** platform:

- Pastors and ministers can **register**, select courses and manage their profile.
- Admins and super admins can:
  - Monitor registrations, verification and payments via the **admin dashboard**.
  - Manage courses and learning plans.
  - Invite additional admins / super admins via secure invite links.
- The backend sends **transactional emails** (e.g. registration and password reset) using `wangamail-rs` (Microsoft Graph / app registration).
- **Microsoft Teams**: Courses can be linked to Teams classes; students see an “Open in Teams” link on their dashboard. See [Microsoft Teams integration](apps/docs/docs/microsoft-teams-integration.md) for enabling Workflows in Teams and creating classes via Graph API.

Everything lives in a **pnpm monorepo** with shared scripts, infra and CI/CD.

---

## Monorepo Structure

```bash
.
├── apps/
│   ├── backend/          # Rust Axum API (auth, courses, admin)
│   ├── frontend/         # SvelteKit frontend (students + admin)
│   └── docs/             # Docusaurus documentation site
├── envs/                 # Local env files + secrets helper          (see envs/README.md)
├── infra/                # Systemd units, scripts, deployment glue   (see infra/systemd/README.md)
├── shared/               # Shared utilities (e.g. shared/utils)      (see shared/utils/README.md)
└── .github/workflows/    # CI/CD for backend + frontend
```

---

## Apps & Services

### Frontend (`apps/frontend`)

- SvelteKit + Tailwind CSS v4.
- Public pages: landing, programs, admissions, events, contact, legal pages.
- Auth flows:
  - Login, register, forgot password, reset password (token-based).
  - Invite-based signup (`/invite/[token]`) for admin / super admin.
- Student dashboard: course selection and progress entry point.
- Admin area:
  - `/admin` overview (metrics, charts).
  - `/admin/users` — registration list and filters.
  - `/admin/courses` — manage courses (name, duration, pricing, active flag).
  - `/admin/learning-plans` — manage learning plans (bundled courses + pricing).
  - `/admin/team` — create invite links for new admins / super admins.

> There is no separate frontend README yet – see the code under `apps/frontend/` for now.

### Backend (`apps/backend`)

Rust Axum API for all server-side concerns:

- Auth:
  - `POST /api/v1/auth/register` — student and invite-based admin signup (roles: `student`, `admin`, `super_admin`).
  - `POST /api/v1/auth/login`
  - `GET /api/v1/auth/me`, `POST /api/v1/auth/verify`, `POST /api/v1/auth/complete-payment`
  - Password reset: request + `POST /api/v1/auth/reset-password`.
- Courses & events:
  - `GET /api/v1/auth/courses`
  - `GET /api/v1/events`
- Contact form:
  - `POST /api/v1/contact`
- Admin API (protected by JWT + role from token):
  - `GET /api/v1/admin/metrics`
  - `GET /api/v1/admin/users`
  - `GET/POST/PATCH /api/v1/admin/courses`
  - `GET/POST/PATCH /api/v1/admin/learning-plans`
  - `POST /api/v1/admin/invites` (super_admin only)
- Email:
  - Registration emails sent via `wangamail-rs` + Microsoft Graph (app registration).

Backend details and lower-level notes live in:

- [`apps/backend/README.md`](apps/backend/README.md)

### Docs (`apps/docs`)

Docusaurus site that will host:

- Public-facing documentation.
- API and admin documentation.
- Deployment and operations guides.

See:

- [`apps/docs/README.md`](apps/docs/README.md)

### Shared & Infra

- **Shared utilities**: `shared/utils` (and future shared modules).
  - See [`shared/utils/README.md`](shared/utils/README.md)
- **Envs & secrets helper**: how to manage `.env` files and push secrets to GitHub:
  - See [`envs/README.md`](envs/README.md)
- **Systemd and deployment**:
  - See [`infra/systemd/README.md`](infra/systemd/README.md) for service units and deployment flow.

---

## Core Features

- **Student-facing**
  - Registration form with ministry details and course selection.
  - Dashboard and course selection.
  - Password reset and secure login.
  - Contact form and events listing.

- **Admin / Super Admin**
  - Metrics dashboard with totals, verification, payments, completion rate.
  - User management: search, pagination, course selection.
  - Course CRUD and learning plan management.
  - Invite system to onboard new admins:
    - Bootstrap super admin via `/invite/<BOOTSTRAP_INVITE_TOKEN>`.
    - Generate invite links for admins or additional super admins.

- **Platform**
  - Rust Axum backend with PostgreSQL (in CI) / SQLite optional in local dev.
  - Node adapter for SvelteKit frontend.
  - GitHub Actions CI/CD with systemd-based deploy for backend.

---

## Local Development

### Prerequisites

- Node.js ≥ 20
- pnpm ≥ 10
- Rust (stable, recent)
- PostgreSQL (recommended for backend dev; CI uses Postgres 16)

### Install & bootstrap

```bash
# From repo root
pnpm install
```

Start services using the Makefile (if present) or pnpm scripts; for example:

```bash
# Frontend dev
cd apps/frontend
pnpm dev

# Backend dev
cd ../backend
cargo run

# Docs dev
cd ../docs
pnpm dev
```

---

## Testing

### Backend

```bash
cd apps/backend
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

### Frontend

```bash
cd apps/frontend
pnpm check          # svelte-check / TS
pnpm lint           # prettier + eslint
pnpm test           # Playwright e2e
```

---

## Environments & Secrets

Local env files:

- `envs/env.env` — repo-level/reusable secrets (synced to GitHub repo secrets).
- `envs/staging.env` — staging environment.
- `envs/prod.env` — production environment.

How to manage and push them:

- See [`envs/README.md`](envs/README.md) for the `cgs` workflow and exact commands.

Important backend envs (repo / environment secrets):

- `DATABASE_URL`
- `APP_BASE_URL`
- `BOOTSTRAP_INVITE_TOKEN`
- `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`
- `WANGAMAIL_FROM_USER`

---

## CI/CD

- **Backend**: `.github/workflows/backend.yml`
  - Format → Lint → Check → Test → Version bump → Build → Deploy (systemd) → Release.
  - Uses Postgres service container for tests and writes `backend.env` on the VM.
- **Frontend**: `.github/workflows/frontend.yml`
  - SvelteKit build, lint, tests.

Failure monitoring jobs open issues under a dedicated **CI/CD Failures** milestone when runs break.

---

## Contributing

Please read [`CONTRIBUTING.md`](CONTRIBUTING.md) for:

- Branch / commit conventions (Conventional Commits).
- How to run tests and builds before opening a PR.
- Expectations for documentation and PR descriptions.

---

## License

This project is licensed under the MIT License – see [`LICENSE`](LICENSE) for full details.

