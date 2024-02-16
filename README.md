# Project Setup Guide

This guide provides instructions on setting up the project, with a focus on initializing the development and test databases and ensuring that SQLx migrations and compile-time SQL verification are correctly integrated.

## Initial Project Setup

1. **Clone the Project**: Start by cloning the project repository to your local machine.

2. **Install Rust and Cargo**: Ensure that [Rust](https://www.rust-lang.org/tools/install) is installed. The project uses Cargo, Rust's package manager and build system, for dependency management and to build the project.

3. **Install SQLx CLI**: The SQLx Command Line Interface (CLI) is used for managing database migrations. Install it globally using Cargo:

   ```sh
   cargo install sqlx-cli
   ```

   This tool is necessary for applying database migrations manually and for generating data for compile-time query validation.

## Database Configuration

The project utilizes two separate SQLite databases: one for development and another for integration testing. This separation ensures that test executions do not interfere with the development database.

### Development Database

1. **Configure Database URL**: Set the `DATABASE_URL` in the `.env` file to point to your development database file, e.g.,

   ```dotenv
   DATABASE_URL=sqlite:./rocket_database.sqlite
   ```

### Test Database

1. **Configure Test Database URL**: Create a `.test.env` file with a different `DATABASE_URL` pointing to your test database file, e.g.,

   ```dotenv
   DATABASE_URL=sqlite:./rocket_database_test.sqlite
   ```

## Applying Migrations

Migrations ensure that your database schema is up to date. Apply migrations to both the development and test databases initially and whenever the schema changes.

1. **Development Database Migrations**: Apply migrations to your development database:

   ```sh
   sqlx database create
   sqlx migrate run
   ```

2. **Test Database Migrations**: Apply migrations to your test database. Ensure to set the `DATABASE_URL` environment variable temporarily to your test database URL:

   ```sh
   DATABASE_URL=sqlite:./rocket_database_test.sqlite sqlx database create
   DATABASE_URL=sqlite:./rocket_database_test.sqlite sqlx migrate run
   ```

## Compile-Time SQL Verification

To enable compile-time verification of SQL queries, ensuring they match the database schema:

1. **Prepare the Project**: With the development database updated, generate `sqlx-data.json` for compile-time verification:

   ```sh
   cargo sqlx prepare -- --lib
   ```

2. **Check in `sqlx-data.json`**: Commit the `sqlx-data.json` file to version control to share the compiled database schema with the team and for continuous integration purposes.

## Running the Project

With the databases set up and migrations applied:

1. **Build and Run**: Use Cargo to build and run the project:

   ```sh
   cargo run
   ```

2. **Running Tests**: Execute integration tests, ensuring they utilize the test database configured in `.test.env`:

   ```sh
   cargo test
   ```