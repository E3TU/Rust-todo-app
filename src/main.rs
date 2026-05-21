use std::io;

fn main() {

    let mut todos: Vec<String> = Vec::new();

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
            }
            "2" => {
                println!("Todos:");
            }
            "3" => {
                println!("Choose a todo to edit");
            }
            "4" => {
                println!("Choose a todo to delete");
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
