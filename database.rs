use dialoguer::Error;
use rusqlite::{Connection, Result};
use tui::widgets::canvas::Label;
use crate::task_class::Task;
use uuid::Uuid;

pub fn create_table(conn: &Connection) -> Result<()>{
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY,
            label TEXT,
            description TEXT
        )",
        [],
    )?;
    Ok(())
}

pub fn add_todo(conn: &Connection, task: &Task) -> Result<()> {
    match begin_transaction(conn) {
        Ok(_) => {
            let result = conn.execute(
                "INSERT INTO todos (id, label, description) VALUES (?1, ?2, ?3)",
                [&task.id.to_string(), &task.label, &task.description],
            );

            match result {
                Ok(_) => {
                    commit_transaction(conn);
                    Ok(())  // Return an Ok result to indicate success
                }
                Err(err) => {
                    rollback_transaction(conn)?;
                    Err(err)  // Return the error if there's a problem
                }
            }
        }
        Err(err) => {
            // Handle the error from begin_transaction here
            Err(err)
        }
    }
}

pub fn get_tasks(conn: &Connection) -> Result<Vec<Result<Task, rusqlite::Error>>>{
    let mut stmt = conn.prepare("SELECT id, label, description FROM todos")?;
    
    let task_iter = stmt.query_map([], |row| {
        let id_str: String = row.get(0)?;
    
        // Try to parse the id as a UUID
        let id = match Uuid::parse_str(&id_str) {
            Ok(uuid) => uuid,
            Err(_) => {
                // If parsing as a UUID fails, check if it's an INTEGER
                match row.get::<usize, i32>(0) {
                    Ok(integer) => {
                        return Err(rusqlite::Error::InvalidColumnType(0, "UUID".to_string(), rusqlite::types::Type::Text ));
                    }
                    Err(_) => {
                        return Err(rusqlite::Error::InvalidColumnType(0, "UUID".to_string(), rusqlite::types::Type::Text ));
                    }
                }
            }
        };
    
        Ok(Task {
            id,
            label: row.get(1)?,
            description: row.get(2)?,
        })
    })?;

    let mut task_vec = Vec::new();
    for task_result in task_iter {
        task_vec.push(task_result);
    }

    Ok(task_vec)
}

fn commit_transaction(conn: &Connection) -> Result<()>{
    conn.execute("COMMIT", [])?;

    Ok(())
}

fn begin_transaction(conn: &Connection) -> Result<()> {
    conn.execute("BEGIN", [])?;
    Ok(())
}

fn rollback_transaction(conn: &Connection) -> Result<()> {
    conn.execute("ROLLBACK", [])?;
    Ok(())
}

pub fn remove_todo(conn: &Connection, label: &str) -> Result<()> {
    match begin_transaction(conn) {
        Ok(_) => {
            let result = conn.execute(
                "DELETE FROM todos WHERE label = ?1",
                (&label,)
            );
            match result {
                Ok(_) => {
                    commit_transaction(conn);
                    Ok(())
                }
                Err(err) => {
                    rollback_transaction(conn)?;
                    eprintln!("This todo does not exist!");
                    Err(err)
                }
            }
        }
        Err(err) => {
            Err(err)
        }
    }
}

pub fn update_todo(conn: &Connection, old_label: &str, new_label: &str, new_description: &str) -> Result<()> {
    match begin_transaction(conn) {
        Ok(_) => {
            let result = conn.execute(
                "UPDATE todos SET label = ?1, description = ?2 WHERE label = ?3",
                (&new_label, &new_description, &old_label)
            );
            match result {
                Ok(_) => {
                    commit_transaction(conn);
                    Ok(())  // Return an Ok result to indicate success
                }
                Err(err) => {
                    rollback_transaction(conn)?;
                    eprintln!("Failed to edit the to-do item: {}", err);
                    Err(err)  // Return the error if there's a problem
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to begin a transaction: {}", err);
            Err(err)
        }
    }
}