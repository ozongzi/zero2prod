# Zero2Prod - Newsletter Application

A production-ready newsletter subscription API built with Rust, Actix-web, and PostgreSQL. This project follows the "Zero to Production in Rust" book methodology to build a robust, testable, and maintainable backend service.

## Features

- **Health Check Endpoint**: Monitor service availability
- **Subscription Management**: RESTful API for newsletter subscriptions
- **Database Integration**: PostgreSQL with SQLx for type-safe queries
- **Configuration Management**: YAML-based configuration with environment-specific settings
- **Structured Logging**: Comprehensive tracing with `tracing` crate
- **CI/CD Pipeline**: GitHub Actions for testing, linting, and code coverage
- **Database Migrations**: Version-controlled schema management with SQLx
- **Security**: Secure password handling with `secrecy` crate

## Tech Stack

- **Rust**: 2024 edition
- **Web Framework**: Actix-web 4
- **Database**: PostgreSQL with SQLx
- **Configuration**: `config` crate with YAML support
- **Logging**: `tracing`, `tracing-subscriber`, `tracing-bunyan-formatter`
- **Testing**: Integration tests with isolated databases
- **CI/CD**: GitHub Actions

## Project Structure

```
zero2prod/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── configuration.rs     # Configuration management
│   ├── telemetry.rs         # Logging and tracing setup
│   ├── startup.rs           # Application startup logic
│   └── routes/
│       ├── mod.rs           # Routes module
│       ├── health_check.rs  # Health check endpoint
│       └── subscriptions.rs # Subscription endpoints
├── tests/
│   └── health_check.rs      # Integration tests
├── migrations/
│   └── 20260208125623_create_subscriptions_table.sql
├── scripts/
│   └── init_db.sh           # Database initialization script
├── .github/workflows/
│   └── general.yml          # CI/CD pipeline
├── configuration.yaml       # Application configuration
├── Cargo.toml              # Rust dependencies
└── Cargo.lock              # Dependency lock file
```

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL 14+
- SQLx CLI (`cargo install sqlx-cli`)

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd zero2prod
   ```

2. Set up the database:
   ```bash
   ./scripts/init_db.sh
   ```

   Or manually:
   ```bash
   # Start PostgreSQL (if using Docker)
   docker run --name zero2prod-postgres \
     -e POSTGRES_USER=postgres \
     -e POSTGRES_PASSWORD=password \
     -e POSTGRES_DB=newsletter \
     -p 5432:5432 \
     -d postgres:14

   # Run migrations
   export DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter
   sqlx database create
   sqlx migrate run
   ```

3. Configure the application:
   ```bash
   cp configuration.yaml.example configuration.yaml
   # Edit configuration.yaml with your settings
   ```

4. Build and run:
   ```bash
   cargo build --release
   cargo run
   ```

   The application will start on `http://127.0.0.1:8000`

### Configuration

The application uses `configuration.yaml` for settings:

```yaml
application_port: 8000
database:
  host: "127.0.0.1"
  port: 5432
  username: "postgres"
  password: "password"
  database_name: "newsletter"
```

## API Endpoints

### Health Check
- **GET** `/health_check`
- Returns: `200 OK` with empty body
- Purpose: Service health monitoring

### Subscriptions
- **POST** `/subscriptions`
- Content-Type: `application/x-www-form-urlencoded`
- Request body:
  ```json
  {
    "name": "John Doe",
    "email": "john@example.com"
  }
  ```
- Returns: `200 OK` on success, `400 Bad Request` for invalid data, `500 Internal Server Error` for database failures
- Purpose: Create a new newsletter subscription

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with logging
TEST_LOG=true cargo test

# Run specific test
cargo test health_check_works
```

### Test Features

- **Isolated Databases**: Each test runs with a unique database
- **Structured Logging**: Optional test logging with `TEST_LOG` environment variable
- **Integration Tests**: Full HTTP request/response testing
- **Database Assertions**: Verify data persistence

## Development

### Database Migrations

```bash
# Create new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert migration
sqlx migrate revert
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Check formatting
cargo fmt --check
```

### Monitoring

The application uses structured logging with the `tracing` crate. Logs include:
- Request/response information
- Database query execution
- Error details with context
- Performance metrics

## CI/CD Pipeline

The GitHub Actions workflow includes:

1. **Test Job**: Runs tests with PostgreSQL service container
2. **Format Check**: Ensures code follows Rustfmt standards
3. **Code Coverage**: Generates coverage reports with `cargo-llvm-cov`

## Security Considerations

- **Password Security**: Database passwords are stored as `SecretString` types
- **Input Validation**: Basic validation for subscription data
- **SQL Injection Prevention**: SQLx provides compile-time query validation
- **Error Handling**: Generic error responses to avoid information leakage

## Deployment

### Building for Production

```bash
cargo build --release
```

The optimized binary will be available at `target/release/zero2prod`

### Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `RUST_LOG`: Logging level (default: `info`)

### Docker Deployment

```dockerfile
FROM rust:latest as builder
WORKDIR /usr/src/zero2prod
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/zero2prod/target/release/zero2prod /usr/local/bin/zero2prod
COPY configuration.yaml /etc/zero2prod/configuration.yaml
EXPOSE 8000
CMD ["zero2prod"]
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Based on the "Zero to Production in Rust" book by Luca Palmieri
- Built with the amazing Rust ecosystem
- Inspired by production-ready backend patterns

## Support

For issues, questions, or contributions:
1. Check existing issues
2. Create a new issue with detailed information
3. Include reproduction steps for bugs

---

**Note**: This is a learning project following production-ready patterns. Use as a reference for building Rust web applications.