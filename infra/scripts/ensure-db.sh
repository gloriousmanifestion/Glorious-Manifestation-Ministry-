#!/usr/bin/env bash
# Ensure the application database exists before starting the backend.
# Usage: ensure-db.sh /path/to/.env
# Reads DATABASE_URL from the .env file. Supports PostgreSQL and SQLite.

set -euo pipefail

ENV_FILE="${1:-}"
if [[ -z "$ENV_FILE" || ! -f "$ENV_FILE" ]]; then
  echo "Usage: $0 /path/to/.env" >&2
  exit 1
fi

# Export DATABASE_URL from .env (simple key=value; no spaces around =)
export $(grep -v '^#' "$ENV_FILE" | grep -E '^DATABASE_URL=' | xargs)

if [[ -z "${DATABASE_URL:-}" ]]; then
  echo "DATABASE_URL not set in $ENV_FILE" >&2
  exit 1
fi

# Normalize postgres:// to postgresql://
DATABASE_URL="${DATABASE_URL/#postgres:\/\//postgresql:\/\/}"

if [[ "$DATABASE_URL" == postgresql://* ]] || [[ "$DATABASE_URL" == postgres://* ]]; then
  # PostgreSQL: create database if it doesn't exist.
  # Parse the URL to get the database name and connection params for 'postgres' DB.
  if ! command -v psql &>/dev/null; then
    echo "psql not found; skipping PostgreSQL ensure-db (backend may create DB or it may already exist)." >&2
    exit 0
  fi
  # Extract database name from path (last path segment)
  DB_NAME=$(echo "$DATABASE_URL" | sed -n 's|.*/\([^/?]*\).*|\1|p')
  # Connection string to default 'postgres' database to run CREATE DATABASE
  CONN_POSTGRES=$(echo "$DATABASE_URL" | sed -E 's|/([^/?]+)(\?.*)?$|/postgres\2|')
  if [[ -z "$DB_NAME" ]]; then
    echo "Could not parse database name from DATABASE_URL" >&2
    exit 1
  fi
  # Create database if not exists (psql returns 0 even when DB exists)
  psql "$CONN_POSTGRES" -tc "SELECT 1 FROM pg_database WHERE datname = '$DB_NAME'" | grep -q 1 || \
    psql "$CONN_POSTGRES" -c "CREATE DATABASE \"$DB_NAME\";"
  echo "PostgreSQL database '$DB_NAME' is ready."
elif [[ "$DATABASE_URL" == sqlite* ]]; then
  # SQLite: ensure parent directory and file exist (backend also does this; harmless).
  PATH_PART="${DATABASE_URL#sqlite:}"
  PATH_PART="${PATH_PART#//}"
  if [[ -z "$PATH_PART" || "$PATH_PART" == "memory"* ]]; then
    exit 0
  fi
  PARENT=$(dirname "$PATH_PART")
  if [[ -n "$PARENT" ]]; then
    mkdir -p "$PARENT"
  fi
  touch "$PATH_PART" 2>/dev/null || true
  echo "SQLite path ensured: $PATH_PART"
else
  echo "Unsupported DATABASE_URL scheme; skipping ensure-db." >&2
fi
exit 0
