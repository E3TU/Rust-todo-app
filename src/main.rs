use std::io;

fn main() {
    struct Todo {
        id: u32,
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
                    id: next_id,
                    text: todo.trim().to_string(),
                });

                next_id += 1;
            }
            "2" => {
                println!("Todos:");

                for todo in &todos {
                    println!("{}: {}", todo.id, todo.text);
                }
                println!("");
            }
            "3" => {
                println!("Choose a todo to edit");
            }
            "4" => {
                println!("Choose a todo to delete by id: ");
                let mut delete = String::new();

                io::stdin()
                    .read_line(&mut delete)
                    .expect("Failed to read input");

                let delete: u32 = delete.trim().parse().expect("Not a valid number");

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
