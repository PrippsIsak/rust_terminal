use::std::io;
use crate::task_class::Task;
use rusqlite::{Connection, Result};
use uuid::Uuid;
use crate::database;
use colored::*;

pub fn running (conn: &Connection) {
    //Initilize database
    check_result(database::create_table(conn));

    let mut exit_flag = true;
    let mut input_user = String::new();

    println!("Welcome to Isak's to do list in terminal");

        while exit_flag {

        io::stdin()
            .read_line(&mut input_user)
            .expect("Failed to read line");

        if input_user.trim() == "add" {
            add(conn);
        }
        else if input_user.trim() == "remove" {
            let _ = delete_todo(conn);
        }

        else if input_user.trim() == "edit" {
            let _ = edit_todo(conn);            
        }

        else if input_user.trim() == "exit" {
            exit_flag = true;
        }

        else if input_user.trim() == "load"{
            print_todos(database::get_tasks(conn))
   
        }
        input_user.clear();
    }
}

fn add(conn: &Connection){

    let mut label_input = String::new();
    let mut description_input = String::new();

    println!(" Add a label");
    io::stdin()
        .read_line(&mut label_input)
        .expect("Failed to read line");

    println!("Add description");
    io::stdin()
        .read_line(&mut description_input)
        .expect("Failed to read line");

    let task = Task::new(
        Uuid::new_v4(),
        label_input.trim().to_string(),
        description_input.trim().to_string(),
    );

    check_result(database::add_todo(conn, &task));
}

fn print_todos(tasks: Result<Vec<Result<Task, rusqlite::Error>>, rusqlite::Error>) {
    match tasks {
        Ok(task_vec) => {
            for result in task_vec {
                match result {
                    Ok(result) => {
                        println!("{:?}", result);
                    }

                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
            }
        }

        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

fn check_result(result:Result<(), rusqlite::Error>){
    match result  {
        Ok(_) => {
            println!("{}","Succes".green())
        }
        Err(_) => {
            println!("{}","Failed".red())
        }
    }
}

fn delete_todo(conn: &Connection) -> Result<()> {
    println!("Which todo do you want to delete?");
    
    let mut input_user = String::new();

    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read line");

    check_result(database::remove_todo(conn, input_user.trim()));
    Ok(())
}

fn edit_todo(conn: &Connection) -> Result<()>{
    let mut old_label = String::new();
    let mut new_label = String::new();
    let mut new_description = String::new();

    println!("Enter the label of the todo you want to change");

    io::stdin()
        .read_line(&mut old_label)
        .expect("Failed to read line");

    println!("Enter new label");

    io::stdin()
        .read_line(&mut new_label)
        .expect("Failed to read line");

    println!("Enter new description");

    io::stdin()
        .read_line(&mut new_description)
        .expect("Failed to read line");

    check_result(database::update_todo(conn, old_label.trim(), new_label.trim(), new_description.trim()));

    Ok(())
}