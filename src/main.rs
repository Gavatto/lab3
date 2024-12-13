use std::fs::{self, File};
use std::io::{self, Write, Read};
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    is_completed: bool,
}

impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            is_completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.is_completed = true;
    }
}

struct TodoApp {
    tasks: Vec<Task>,
}

impl TodoApp {
    fn new() -> TodoApp {
        TodoApp { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        } else {
            println!("Error: Task index out of range.");
        }
    }

    fn edit_task(&mut self, index: usize, new_description: &str) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.description = new_description.to_string();
        } else {
            println!("Error: Task index out of range.");
        }
    }

    fn mark_task_completed(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.mark_completed();
        } else {
            println!("Error: Task index out of range.");
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let serialized_tasks = serde_json::to_string(&self.tasks)?;
        let mut file = File::create(filename)?;
        file.write_all(serialized_tasks.as_bytes())?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<TodoApp> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let tasks: Vec<Task> = serde_json::from_str(&contents)?;
        Ok(TodoApp { tasks })
    }

    fn show_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else {
            for (index, task) in self.tasks.iter().enumerate() {
                let status = if task.is_completed { "Completed" } else { "Not completed" };
                println!("{}. {} [{}]", index + 1, task.description, status);
            }
        }
    }
}

fn main() {
    let mut app = TodoApp::new();
    let filename = "tasks.json";

    // Завантажуємо завдання з файлу
    if let Ok(loaded_app) = TodoApp::load_from_file(filename) {
        app = loaded_app;
    }

    loop {
        println!("\nTodo List Menu:");
        println!("1. Show tasks");
        println!("2. Add task");
        println!("3. Edit task");
        println!("4. Remove task");
        println!("5. Mark task as completed");
        println!("6. Save and exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                app.show_tasks();
            }
            2 => {
                let mut description = String::new();
                println!("Enter task description:");
                io::stdin().read_line(&mut description).expect("Failed to read line");
                app.add_task(&description.trim());
            }
            3 => {
                let mut index = String::new();
                println!("Enter task index to edit:");
                io::stdin().read_line(&mut index).expect("Failed to read line");
                let index: usize = index.trim().parse().unwrap_or(0);

                let mut new_description = String::new();
                println!("Enter new task description:");
                io::stdin().read_line(&mut new_description).expect("Failed to read line");
                app.edit_task(index - 1, &new_description.trim());
            }
            4 => {
                let mut index = String::new();
                println!("Enter task index to remove:");
                io::stdin().read_line(&mut index).expect("Failed to read line");
                let index: usize = index.trim().parse().unwrap_or(0);
                app.remove_task(index - 1);
            }
            5 => {
                let mut index = String::new();
                println!("Enter task index to mark as completed:");
                io::stdin().read_line(&mut index).expect("Failed to read line");
                let index: usize = index.trim().parse().unwrap_or(0);
                app.mark_task_completed(index - 1);
            }
            6 => {
                if let Err(e) = app.save_to_file(filename) {
                    println!("Error saving tasks: {}", e);
                } else {
                    println!("Tasks saved successfully.");
                }
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}
