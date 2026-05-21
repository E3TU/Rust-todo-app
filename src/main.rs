use std::io;

fn main() {
    struct Todo {
        id: String,
        text: String,
    }

    let mut todos: Vec<Todo> = Vec::new();

    let mut next_id: u32 = 1;

    loop {
        println!("1. Add todo");
        println!("2. List todos");
        println!("3. Edit todo");
        println!("4. Delete todos");
        println!("5. Quit program");

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter a todo: ");

                let mut todo = String::new();

                io::stdin()
                    .read_line(&mut todo)
                    .expect("Failed to read input");

                let todo = todo.trim().to_string();

                todos.push(Todo {
                    id: next_id.to_string(),
                    text: todo.trim().to_string(),
                });

                next_id += 1;
            }
            "2" => {
                println!("Todos:");

                for (i, todo) in todos.iter().enumerate() {
                    println!("{}. {} (id: {})", i + 1, todo.text, todo.id);
                }
                println!("");
            }
            "3" => {
                println!("Choose a todo to edit by id: ");
                let mut edit = String::new();

                io::stdin()
                    .read_line(&mut edit)
                    .expect("Failed to read input");

                let edit = edit.trim();

                println!("Enter new text: ");
                let mut new_text = String::new();

                io::stdin().read_line(&mut new_text).expect("Failed to read input");

                let new_text = new_text.trim().to_string(); 

                if let Some(todo) = todos.iter_mut().find(|t| t.id == edit) {
                    todo.text = new_text;
                } else {
                    println!("Todo with ID '{}' not found.", edit)
                }
            }
            "4" => {
                println!("Choose a todo to delete by id: ");
                let mut delete = String::new();

                io::stdin()
                    .read_line(&mut delete)
                    .expect("Failed to read input");

                let delete = delete.trim();

                println!("Deleting todo with id: {delete}");

                todos.retain(|todo| todo.id != delete);
            }
            "5" => {
                break;
            }
            _ => {
                println!("Invalid option");
            }
        }
    }
}
