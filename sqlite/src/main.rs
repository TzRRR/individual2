use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use sqlite::{create_table, drop_table, load_data_from_csv, query_exec};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new table with the airline schema
    #[command(alias = "c", short_flag = 'c')]
    Create { table_name: String },

    /// Execute a query
    #[command(alias = "q", short_flag = 'q')]
    Query { query: String },

    /// Drop an existing table
    #[command(alias = "d", short_flag = 'd')]
    Delete { delete_query: String },

    /// Load data from CSV file into the table
    #[command(alias = "l", short_flag = 'l')]
    Load {
        table_name: String,
        file_path: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let conn = Connection::open("airline_database.db")?;

    match args.command {
        Commands::Create { table_name } => {
            println!("Creating Table '{}'", table_name);
            create_table(&conn, &table_name).expect("Failed to create table");
        }
        Commands::Query { query } => {
            println!("Executing Query: {}", query);
            query_exec(&conn, &query).expect("Failed to execute query");
        }
        Commands::Delete { delete_query } => {
            println!("Dropping Table '{}'", delete_query);
            drop_table(&conn, &delete_query).expect("Failed to drop table");
        }
        Commands::Load {
            table_name,
            file_path,
        } => {
            println!(
                "Loading data into table '{}' from '{}'",
                table_name, file_path
            );
            load_data_from_csv(&conn, &table_name, &file_path)
                .expect("Failed to load data from CSV");
        }
    }
    Ok(())
}
