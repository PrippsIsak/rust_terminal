pub mod terminal;
pub mod task_class;
mod database;

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    terminal::running(&conn);
    Ok(())
}