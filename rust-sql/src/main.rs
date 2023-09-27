/*
Rust CLI for SQLite CRUD operations
*/

use clap::Parser;
use rusqlite::{Connection, Result};

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Kahlia Hogg",
    about = "A CLI tool SQLite",
    after_help = "Example: cargo run create --db <db_name> --query <query>"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Execute {
        #[clap(long)]
        db: String,
        #[clap(short, long)]
        q: String,
    },
    Insert {
        #[clap(long)]
        db: String,
        #[clap(long)]
        table: String,
        #[clap(long)]
        data: String,
    },
    Read {
        #[clap(long)]
        db: String,
        #[clap(long)]
        table: String,
    },
    Drop {
        #[clap(long)]
        db: String,
        #[clap(long)]
        table: String,
    },
}

// Main fxn returns result or error
fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Execute { db, q }) => {
            let db_name = db + ".db";
            let conn = Connection::open(db_name)?;
            conn.execute(&q, ())?;
            println!("Query {:?} executed successfully", q);
            Ok(())
        }
        Some(Commands::Insert { db, table, data }) => {
            let db_name = db + ".db";
            let conn = Connection::open(db_name)?;
            let query = format!("INSERT INTO {} VALUES {};", table, data);
            conn.execute(&query, ())?;
            println!("Query {:?} executed successfully", query);
            Ok(())
        }
        Some(Commands::Read { db, table }) => {
            let db_name = db + ".db";
            let conn = Connection::open(db_name)?;
            let query = format!("SELECT * FROM {}", table);
            let mut stmt = conn.prepare(&query)?;
            // let cols = stmt.column_names();
            let mut rows = stmt.query([])?;
            println!("Table: {:?}:", table);
            while let Some(row) = rows.next()? {
                println!("{:?}", row)
            }
            Ok(())
        }
        Some(Commands::Drop { db, table }) => {
            let db_name = db + ".db";
            let conn = Connection::open(db_name)?;
            let query = format!("DROP TABLE {}", table);
            conn.execute(&query, ())?;
            println!("Table: {:?} dropped successfully", table);
            Ok(())
        }
        None => {
            println!("No command specified");
            Ok(())
        }
    }
}
