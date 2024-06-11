# Simple Rust Identity Service

I wanted to learn how to build a simple yet non-trivial API service in Rust, so this repo contains the code for a simple local-account identity service. It is built using the following frameworks and libraries:

- [tokio](https://docs.rs/tokio/latest/tokio/) for the async runtime
- [axum](https://docs.rs/axum/latest/axum/) and [tower](https://docs.rs/tower/latest/tower/) for the web framework
- [sqlx](https://docs.rs/sqlx/latest/sqlx/) for database access 
- [postgres](https://hub.docker.com/_/postgres) for the runtime database, and [dashmap](https://docs.rs/dashmap/latest/dashmap/) for the test fake
- [argon2](https://docs.rs/argon2/latest/argon2/) for password hashing
- [thiserror](https://docs.rs/thiserror/latest/thiserror/) for error types
- [chrono](https://docs.rs/chrono/latest/chrono/) for timestampts
- [axum-test](https://docs.rs/axum-test/latest/axum_test/) for easier API testing

This service is fully-functional, but for educational purposes only, so it shouldn't be used in a production system without a security review and more testing.

## APIs

The service implements the following APIs:

| Method | Path | Description | Request Body | Response Body
|--------|------|-------------|--------------|--------------
| POST | /accounts | Creates a new local account | [NewAccountRequest](./src/api/models.rs) | [AccountResponse](./src/api/models.rs) or BAD_REQUEST error
| POST | /sessions | Authenticates provided credentials | [AuthenticationRequest](./src/api/models.rs) | [AccountResponse](./src/api/models.rs) or BAD_REQUEST error

A caller such as an API gateway could use these APIs to support sign-up/in. During sign-in, the API gateway would use this service to authenticate the credentials, create a new digitally-signed session token, and put the account details into a cache like [redis](https://redis.io/) using the session token as the key, and drop the session token as a response cookie. Subsequent requests that include the cookie would be treated as authenticated if the token's signature is still valid.

## Architecture

The architecture and code organization I used might be a tad overkill for such a simple service, but I wanted to work out an approach that could scale up to large monoliths with several internal but isolated services, and multiple types of APIs (REST, gRPC, WebSockets, GraphQL, etc).

The architecture is divided into layers:

- **API Layer:** This is a relatively thin layer that is responsible only for the semantics of the API protocol and contract--all the real work happens in the service layer. For example, the API layer is concerned with things like JSON \[de]serialization and HTTP status codes, but not data validation, business logic, or data storage. This layer defines models for API requests and responses, but those are separate from those defined at the Service layer so that the APIs can evolve independently of the services. This layer can support multiple kinds of APIs at the same time (REST, gRPC, WebSockets, etc) each of which interacts with the same set of internal services.
- **Service Layer:** Responsible for enforcing all the business logic and interacting with the data stores. This layer can include multiple services, but they remain isolated from each other so that services can ensure data integrity and do intelligent caching. For example, if service A wants data from service B, it must go through the public service B interface, and not directly to the service's data store.
- **Store Layer:** Responsible only for data storage and retrieval. This is a relatively thin layer that simply interacts with the database to insert, update, delete, and read data. Each service typically defines a [trait](./src/services/account/store.rs) for its data store, which can be implemented for different kinds of databases (e.g., PostgreSQL, MongoDB, DynamoDB, Aurora, Spanner, etc). This trait is also implemented on a [fake](./src/services/account/store/fake.rs) for unit testing.

Lower layers have not knowledge of the layers above them. For example, Stores have no knowledge of Services or APIs, but do necessarily know about the Database they are talking to.

## Local Development

The service is currently configured to use PostgreSQL as the runtime database. The easiest way to run a local Postgres instance is to use [Docker](https://www.docker.com/). Install the [Docker Desktop](https://www.docker.com/products/docker-desktop/) for you operating system. Then run this at the command line to build and run a local PostgreSQL container that will automatically create the required table as the server starts:

```bash
# required password for the local postgres instance
export POSTGRES_PASSWORD=my-local-postgres-password
docker compose up -d
```

Make sure the container didn't exit with an error by either looking at the Docker Desktop dashboard, or by running `docker ps -a` at the command line. If the container is stopped, something went wrong--check the logs to see what happened.

To run the service, run these commands in the same terminal where you set the `POSTGRES_PASSWORD` environment variable:

```bash
export ADDR=127.0.0.1:3000
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
