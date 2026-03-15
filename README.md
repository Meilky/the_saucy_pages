# The Saucy Pages

## Requirements

You need to install the
[sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) to create
and run the migrations on your database.

## Setup

- The `.env`

```bash
cp .env.example .env
```

- The database

```bash
sqlx database create
sqlx migrate run
```

## Starting

```bash
cargo run
```
