# Rust CLI with SQLite Integration

This project is a command-line interface (CLI) application built in Rust that manages data stored in an SQLite database. It supports key operations including creating tables, executing queries, deleting tables, and loading data from CSV files.

## Features

- **Create**: Initialize tables with specific schemas.
- **Query**: Run SQL commands to read or update data.
- **Delete**: Drop tables as needed.
- **Load Data**: Import data directly from a CSV file.

## Usage

1. **Build the Project**:

   ```bash
   cargo build --release

   ```

2. **Add the Binary to PATH (optional)**:

   ```bash
   export PATH=$PATH:/path/to/target/release
   ```

3. **Run Commands**:

   ```bash
   sqlite -c table_name       # Create a table
   sqlite -q "SELECT * FROM table_name" # Execute a query
   sqlite -d table_name       # Delete a table
   sqlite -l table_name /path/to/file.csv # Load data from CSV

   ```

## Development and Testing

Refer to the Makefile.

## Use of LLM in Development

During the development of this Rust CLI project, I consulted a Large Language Model (LLM) to help streamline and clarify various aspects of the coding process. Here are some ways in which the LLM contributed:

1. **Syntax and Code Structure Guidance**:

   - I used the GitHub Copilot to explain Rust syntax and best practices, such as structuring `lib.rs` and `main.rs` files, using modules effectively, and understanding Rust's error-handling conventions.
   - The Copilot provided guidance on Rust-specific commands, such as `cargo fmt` for automatic code formatting, and helped configure an optimized CI/CD setup using GitHub Actions.

2. **File Structure and Workflow Setup**:

   - The Copilot assisted in creating a clear and modular file structure, including suggestions for organizing the `Makefile` for CI purposes and structuring `lib.rs` for improved functionality and testing.
   - Additionally, I received detailed explanations of various configuration elements in the `CI.yml` file, such as `working-directory` syntax, environment variable setup, and artifact uploads in GitHub Actions.

3. **Sample Code and Testing**:
   - When working with SQLite integration and the CSV loader, the Copilot helped generate sample test data and provided templates for automated tests and guidance on using Rust’s `cargo test` and Clippy for linting and testing best practices.

Overall, consulting the GitHub Copilot significantly improved my understanding of Rust and enhanced the code structure, readability, and workflow automation in this project.

## Enjoy!
