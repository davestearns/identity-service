# Simple Rust Identity Service

[![Rust](https://github.com/davestearns/identity-service/actions/workflows/rust.yml/badge.svg)](https://github.com/davestearns/identity-service/actions/workflows/rust.yml)

I wanted to learn how to build a simple yet non-trivial API service in Rust, so this repo contains the code for a simple local-account identity service. It is built using the following frameworks and libraries:

- [tokio](https://docs.rs/tokio/latest/tokio/) for the async runtime
- [axum](https://docs.rs/axum/latest/axum/) and [tower](https://docs.rs/tower/latest/tower/) for the web framework
- [sqlx](https://docs.rs/sqlx/latest/sqlx/) for database access 
- [postgres](https://hub.docker.com/_/postgres) for the runtime database
- [argon2](https://docs.rs/argon2/latest/argon2/) for password hashing
- [thiserror](https://docs.rs/thiserror/latest/thiserror/) for error types
- [chrono](https://docs.rs/chrono/latest/chrono/) for timestampts
- [axum-test](https://docs.rs/axum-test/latest/axum_test/) for easier API testing

## APIs

The service implements the following APIs:

| Method | Path | Description | Request Body | Response Body
|--------|------|-------------|--------------|--------------
| POST | /accounts | Creates a new local account | [NewAccountRequest](./src/api/models.rs) | [AccountResponse](./src/api/models.rs) or BAD_REQUEST error
| PUT | /accounts/:id/credentials | Updates account credentials | [UpdateCredentialsRequest](./src/api/models.rs) | [AccountResponse](./src/api/models.rs) or BAD_REQUEST error
| POST | /sessions | Authenticates provided credentials | [AuthenticationRequest](./src/api/models.rs) | [AccountResponse](./src/api/models.rs) or BAD_REQUEST error

A caller such as an API gateway could use these APIs to support basic sign-up/in and updating credentials. During sign-in, the API gateway would use this service to authenticate the credentials, create a new digitally-signed session token, put the account details into a cache like [redis](https://redis.io/) using the session token as the key, and drop the session token as a response cookie. When the API gateway receives a subsequent request containing the cookie, it would validate the token's signature to ensure it wasn't tampered with or forged, and fetch the user profile from the cache if it all checks out.

Although this service is functional, it was built for educational purposes only, so it shouldn't be used in a production system without further modifications and review. Specifically, the following features are not yet implemented:

- Account deactivation
- Passkeys
- Audit log with events about updates to accounts
- Authorizing through other identity providers (e.g., sign in with Google/GitHub/Apple/etc)
- Runtime metrics via Prometheus or statsd

## Architecture

The architecture and code organization I used might be a tad overkill for such a simple service, but I wanted to work out a pattern that could scale up to large monoliths with several internal but isolated services, supporting multiple types of APIs (REST, gRPC, WebSockets, GraphQL, etc).

The architecture is divided into layers:

```
+--------------------------------+
|              APIs              |
+--------------------------------+
|                                |
|            Services            |
|                                |
+--------------------------------+
|             Stores             |
+--------------------------------+
```

- **API Layer:** This is a relatively thin layer that is responsible only for the semantics of the API protocol and contract--all the real work happens in the service layer. For example, the API layer is concerned with things like JSON \[de]serialization and HTTP status codes, but not data validation, business logic, or data storage. This layer defines models for API requests and responses, but those are separate from those defined at the Service layer so that the APIs can evolve independently of the services. This layer can support multiple kinds of APIs at the same time (REST, gRPC, WebSockets, etc) each of which interacts with the same set of internal services.
- **Service Layer:** There is where all the business logic is enforced and all the significant work gets done. This layer can include multiple services, but they remain isolated from each other so that services can ensure data integrity and do intelligent caching. For example, if service A wants something from service B, it must go through service B's public interface, and not directly to the service's tables in the data store.
- **Store Layer:** This is a relatively thin layer that simply interacts with the target database to insert, update, delete, and read data. Each service typically defines a [trait](./src/services/account/store.rs) for its data store, which can be implemented for different kinds of databases (e.g., PostgreSQL, MongoDB, DynamoDB, Aurora, Spanner, etc). This trait is also implemented by a [fake](./src/services/account/store/fake.rs) that can be used for unit testing.

Lower layers have no knowledge of the layers above them. For example, Stores have no knowledge of Services or APIs, but do necessarily know about the Database they are talking to.

## Code Organization

Under the `src` directory, the code is divided into `src/apis` and `src/servcies`. The former is where all the code for the API layer lives, and the latter contains services and their related stores.

Rust doesn't seem to have a strong opinion about module names being singular or plural, so I went with the advice given [here](https://users.rust-lang.org/t/pluralization-in-apis-guideline/66233), which says to use singular when there is only one main thing being exported, and plural when there are (or could be in the future) several.

```bash
src/
  main.rs           # main() fn, dependency injection
  error.rs          # StartupError
  apis.rs           # main module for all APIs
  apis/
    error.rs        # ApiError
    converters.rs   # From<...> impls for service models
    models.rs       # common API models
    rest.rs         # REST API impl
  services.rs       # main module for all services
  services/
    account.rs      # AccountService impl
    account/
      error.rs      # AccountServiceError enum and converters
      models.rs     # AccountService models
      stores.rs     # AccountStore trait
      stores/
        error.rs    # AccountStoreError enum
        postgres.rs # PostgresAccountStore impl
        fake.rs     # FakeAccountStore impl
```

Errors and models could, of course, be included in the service implementation file, but I found that splitting them into separate files kept the clutter down. Each `models.rs` mostly contains `struct` definitions, and `error.rs` is just the error enum along with `From<...>` implementations that convert from errors returned from lower layers.

I also used `From<...>` traits to convert between API models to service models. These are defined in the API layer so that the service layer remains ignorant of the API layer models (love how Rust lets you implement a trait on a type defined in a different module!). This allows the API code to simply call `.into()` then it needs to convert to/from a service model. This looks very clean, but there is a tradeoff in readability/discoverability: a new engineer looking at the code might not know why that `.into()` works, and where the associated code is defined. Jump to source doesn't really help since that jumps to the `.into()` method implementation. Perhaps the rust-analyzer plugin in VSCode will someday offer a "Jump to From<...> implementation" command?

## Local Development

The service is currently configured to use PostgreSQL as the runtime database. The easiest way to run a local Postgres instance is to use [Docker](https://www.docker.com/). Install the [Docker Desktop](https://www.docker.com/products/docker-desktop/) for you operating system. Then run this at the command line to build and run a local PostgreSQL container that will automatically create the required table as the server starts:

```bash
# required password for the local postgres instance
export POSTGRES_PASSWORD=...some password you want to use...
docker compose up -d
```

Make sure the container didn't exit with an error by either looking at the Docker Desktop dashboard, or by running `docker ps -a` at the command line. If the container is stopped, something went wrong--check the logs to see what happened.

To run the service, run these commands in the same terminal where you set the `POSTGRES_PASSWORD` environment variable:

```bash
export REST_ADDR=127.0.0.1:3000
export POSTGRES_URL=postgres://postgres:${POSTGRES_PASSWORD}@localhost
cargo run
```

Alternatively, you can create a file in the repo root named `.env` and put those two `export` commands into it. This will set those environment variables automatically each time you run the service.

You can then use a tool like [Postman](https://www.postman.com/) or good ol' `curl` to make requests against the API:

```bash
# create a new account
curl -X POST -H "Content-Type: application/json" \
-d '{"email":"test@test.com","password":"test-password"}' \
http://localhost:3000/accounts

# authenticate that account
curl -X POST -H "Content-Type: application/json" \
-d '{"email":"test@test.com","password":"test-password"}' \
http://localhost:3000/sessions
```

When you're finished, run this to stop and delete the Docker container and network:

```bash
docker compose down
```

The data being stored by the Postgres container lives inside the container so this will also destroy all the data. If you want to preserve data between runs, adjust the [compose.yaml](./compose.yaml) file to include a [volume mount](https://docs.docker.com/compose/compose-file/05-services/#volumes) that maps `/var/lib/postgresql/data` in the container to a file on your host's drive.

If you already have an existing Postgres instance in a cloud provider and you want to use it instead, run the [schema creation script](./docker/postgres/schema.sql) in one of the databases, and set the `POSTGRES_URL` environment variable to point toward your instance.
