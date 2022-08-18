# Quay

Quay is an open source, high performance backend for the Seaport smart 
contracts. The project is implemented in Rust, using Postgres as a storage
backend, with the aim of allowing a proliferation of signature based 
trading platforms for ERC-721s and ERC-1155s based on the Seaport smart 
contract interface.

## Project structure

### Web application

`main.rs` contains the `main` function, which is called when the crate's
binary is executed. That in turn calls `startup.rs`. That creates the
application, attaches the services, routes, database connection pool, and
rpc connection context used by the endpoints.

### Routes/Services

Routes and services are defined in `/routes` using `actix-web` macros.

### Telemetry

`telemetry.rs` and `startup.rs` set up a tracing framework that provides rich
logs for runtime telemetry and development debugging. These tracing logs are
then extended via macros on routes.

Example:

```rust
#[post("/offers")]
#[tracing::instrument(
name = "Adding a new offer",
skip(offer, pool, seaport),
fields(
offerer = %offer.parameters.offerer,
)
)]
async fn create_offer(
    offer: web::Json<Order>,
    pool: web::Data<PgPool>,
    seaport: web::Data<Seaport<Provider<ethers::providers::Http>>>,
) -> HttpResponse {
    if insert_offer(&pool, &offer, &seaport).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}
```

### Database

The database schema and queries are written in SQL and managed with `sqlx`. The
schemas live in `migrations/`, and the database queries are inline where used
on the objects (SQL speaking objects). The database connection is handled in a
connection pool at application startup.

#### Generating queries for macros

When new queries are added, run `DATABASE_URL=postgres://postgres:password@localhost:5432/quay cargo sqlx prepare -- --lib`
to generate new query definitions for the macro expansion used by sqlx.

#### Migrations

Database schema migrations are performed out of band with
[sqlx-cli](https://crates.io/crates/sqlx-cli). All database migrations should be
tested locally, and applied in a non-breaking fashion. What this means, ideally,
is that there is a migration to add new functionality, then a rollout of the new
backend version, then another migration to delete any legacy schema after roll
out.

##### How to run migration using `sqlx-cli`
1. `sqlx migrate add '<MIGRATION_NAME>'`;
2. Write all necesarry queries in `migrations/<MIGRATION_NAME>`;
3. Apply for local db (if it's required for testing): `sqlx migrate run`;

### Smart contract interactions and data structures

Blockchain interactions happen via RPC using the `ethers-rs` library. This project
contains type safe bindings for the seaport (`seaport.rs`) and conduit controller
(`conduit_controller.rs`) contracts, which have been extended with other
functionality, and will eventually include functions for serializing to and
from postgres. The rpc connection is handled by a wrapped reference counter
at application startup.

### Local development environment

Developers will need rust and docker to work on this project. Docker is used
to host a local instance  of the postgres database to execute tests against.
To set up a local database, run: `./scripts/init_db.sh`.

#### Building the project

Run `cargo build` to build the project.

### Tests

Tests live in the `/tests` folder.

#### Running the tests

Run `cargo test` any time after setting up your development environment.

## CI/CD

CI/CD is implemented using GitHub actions, the scripting is in
`.github/workflows`. There are certain actions which run on open
pull requests, and deploy actions, which run only after passing
tests on `master`.

### Continuous Integration

This project uses `clippy`, `tarpaulin`, and a number of other crates
to provide end-to-end tests, unit tests, formatting, linting, and static
analysis to enforce code quality. These tests are then run in an automated
fashion using GitHub actions.

### Continuous Deployment

This project supports deployment to Digital Ocean via `spec.yaml`