struct Task {
    description: String,
    completed: bool,
}

fn main() {
    println!("Bem vindo ao TODO list!");

    let mut tasks: Vec<Task> = Vec::new();

    tasks.push(Task {
        description: String::from("Aprender Rust"),
        completed: false,
    });

    tasks.push(Task {
        description: String::from("construir o projeto todo list"),
        completed: true,
    });

    list_tasks(&tasks);
}

fn list_tasks(tasks: &[Task]) {
    println!("\n--- Sua Lista de Tarefas ---");
    if tasks.is_empty() {
        println!("Você não tem tarefas. Adicione uma!");
    } else {
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.completed { "[x]" } else { "[ ]" };
            println!("{} {} - {}", i+1, status, task.description);
        }
    }
    println!("---------------------------\n");
}
