# matsunoki

Wiki engine

## Development

### Development required tools

- rust
- docker
- psqldef
- nodejs

### Starting frontend server(preview environment)

1. Exec `npm ci`
1. Exec `npm run start`
1. Access [localhost:3001](https://localhost:3001) with a browser

### Starting frontend SSR server

TBD

### Starting backend api server

1. Run the following shellscript to set the environment variables.

```
export ACCOUNT_DB_USER="development"
export ACCOUNT_DB_PASSWORD="development"
export ACCOUNT_DB_HOST="localhost"
export ACCOUNT_DB_PORT="5432"
export ACCOUNT_DB_NAME="matsunoki-account"
export DATABASE_URL="postgres://development:development@localhost:5432/matsunoki-account"
export ACCOUNT_FIREBASE_PROJECT_ID=<Your firebase AccountID>
export ACCOUNT_DB_MAX_CONNECTIONS=5
```

1. Exec cargo run --bin account-http
