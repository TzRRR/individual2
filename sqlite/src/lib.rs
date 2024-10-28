use csv::ReaderBuilder;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File;

// Function to create a table with the airline dataset schema
pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            airline TEXT NOT NULL,
            avail_seat_km_per_week INTEGER NOT NULL,
            incidents_85_99 INTEGER NOT NULL,
            fatal_accidents_85_99 INTEGER NOT NULL,
            fatalities_85_99 INTEGER NOT NULL,
            incidents_00_14 INTEGER NOT NULL,
            fatal_accidents_00_14 INTEGER NOT NULL,
            fatalities_00_14 INTEGER NOT NULL
        )",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(())
}

// Function to execute a query and print results
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    let mut stmt = conn.prepare(query_string)?;

    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let airline: String = row.get(1)?;
        let avail_seat_km_per_week: i64 = row.get(2)?;
        let incidents_85_99: i32 = row.get(3)?;
        let fatal_accidents_85_99: i32 = row.get(4)?;
        let fatalities_85_99: i32 = row.get(5)?;
        let incidents_00_14: i32 = row.get(6)?;
        let fatal_accidents_00_14: i32 = row.get(7)?;
        let fatalities_00_14: i32 = row.get(8)?;
        Ok((
            id,
            airline,
            avail_seat_km_per_week,
            incidents_85_99,
            fatal_accidents_85_99,
            fatalities_85_99,
            incidents_00_14,
            fatal_accidents_00_14,
            fatalities_00_14,
        ))
    })?;

    for row in rows {
        let (
            id,
            airline,
            avail_seat_km_per_week,
            incidents_85_99,
            fatal_accidents_85_99,
            fatalities_85_99,
            incidents_00_14,
            fatal_accidents_00_14,
            fatalities_00_14,
        ) = row?;
        println!(
            "ID: {}, Airline: {}, Available Seat-Km: {}, Incidents 85-99: {}, Fatal Accidents 85-99: {}, Fatalities 85-99: {}, Incidents 00-14: {}, Fatal Accidents 00-14: {}, Fatalities 00-14: {}",
            id, airline, avail_seat_km_per_week, incidents_85_99, fatal_accidents_85_99, fatalities_85_99, incidents_00_14, fatal_accidents_00_14, fatalities_00_14
        );
    }
    Ok(())
}

// Function to drop the table
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

// Function to load data from CSV into the table
pub fn load_data_from_csv(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let insert_query = format!(
        "INSERT INTO {} (airline, avail_seat_km_per_week, incidents_85_99, fatal_accidents_85_99, fatalities_85_99, incidents_00_14, fatal_accidents_00_14, fatalities_00_14) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        table_name
    );

    for result in rdr.records() {
        let record = result?;
        conn.execute(
            &insert_query,
            params![
                &record[0],
                record[1].parse::<i64>()?,
                record[2].parse::<i32>()?,
                record[3].parse::<i32>()?,
                record[4].parse::<i32>()?,
                record[5].parse::<i32>()?,
                record[6].parse::<i32>()?,
                record[7].parse::<i32>()?
            ],
        )?;
    }

    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    /// Helper function to create an in-memory database connection for testing.
    fn setup_test_db() -> Connection {
        Connection::open_in_memory().expect("Failed to create in-memory database")
    }

    #[test]
    fn test_create_table() {
        let conn = setup_test_db();
        let table_name = "test_table";

        let result = create_table(&conn, table_name);
        assert!(result.is_ok(), "Failed to create table");

        // Verify that the table exists by querying the SQLite system tables
        let table_exists: bool = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
            .unwrap()
            .exists([table_name])
            .unwrap();

        assert!(
            table_exists,
            "Table '{}' does not exist after creation",
            table_name
        );
    }

    #[test]
    fn test_load_data_from_csv() {
        let conn = setup_test_db();
        let table_name = "test_airline_data";

        // Create the table with the same structure as the CSV data
        create_table(&conn, table_name).expect("Failed to create table");

        // Path to a sample CSV file for testing
        let csv_path = "../test_airline.csv"; // Replace this with an actual test CSV file path

        // Attempt to load data from the CSV
        let result = load_data_from_csv(&conn, table_name, csv_path);
        assert!(result.is_ok(), "Failed to load data from CSV");

        // Verify that data was loaded by counting rows in the table
        let row_count: i32 = conn
            .prepare(&format!("SELECT COUNT(*) FROM {}", table_name))
            .unwrap()
            .query_row([], |row| row.get(0))
            .unwrap();

        assert!(
            row_count > 0,
            "No rows loaded into '{}' from CSV",
            table_name
        );
    }

    #[test]
    fn test_drop_table() {
        let conn = setup_test_db();
        let table_name = "test_table";

        // Create the table first
        create_table(&conn, table_name).expect("Failed to create table");

        // Drop the table
        let result = drop_table(&conn, table_name);
        assert!(result.is_ok(), "Failed to drop table");

        // Verify that the table no longer exists
        let table_exists: bool = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
            .unwrap()
            .exists([table_name])
            .unwrap();

        assert!(
            !table_exists,
            "Table '{}' still exists after deletion",
            table_name
        );
    }

    #[test]
    fn test_query_exec() {
        let conn = setup_test_db();
        let table_name = "test_airline_data";

        // Create and populate the table
        create_table(&conn, table_name).expect("Failed to create table");

        // Insert some test data matching the CSV structure
        conn.execute(
            &format!(
                "INSERT INTO {} (airline, avail_seat_km_per_week, incidents_85_99, fatal_accidents_85_99, fatalities_85_99, incidents_00_14, fatal_accidents_00_14, fatalities_00_14) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                table_name
            ),
            params![
                "Sample Airline",
                1000000000,
                5,
                1,
                50,
                3,
                0,
                0
            ]
        ).expect("Failed to insert test data");

        // Execute a select query
        let query = format!("SELECT * FROM {}", table_name);
        let result = query_exec(&conn, &query);
        assert!(result.is_ok(), "Failed to execute query");

        // If we reach this point, query_exec has successfully printed the results
    }
}
