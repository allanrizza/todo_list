use std::io;
struct Task {
    description: String,
    completed: bool,
}

fn main() {
    
    println!("Bem vindo ao TODO list!");

    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\nComandos: list, add <descrição>, done <id>, remove <id>, sair\n");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a entrada do usuário");

        let mut parts = input.trim().split_whitespace();

        let command = parts.next().unwrap_or("");

        if command == "sair" {
            println!("Saindo...");
            break;
        } else if command == "list" {
            list_tasks(&tasks);
        } else if command == "add" {
            let description: String = parts.collect::<Vec<&str>>().join(" ");
            if description.is_empty() {
                println!("a descrição não pode estar vazia");
            } else {
                println!("adicionando tarefa: {}", description);
                add_task(&mut tasks, description);
            }
        } else if command == "done" {
            if let Some(id_str) = parts.next() {
                match id_str.parse::<usize>() {
                    Ok(id) => {
                        mark_task_done(&mut tasks, id);
                    }
                    Err(_) => {
                        println!("ERRO: O ID fornecido '{}' não é um número válido.", id_str);
                    }
                }
            }
        } else {
            println!("comando desconhecido: {}", command);
        }       
    }
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

fn add_task(tasks: &mut Vec<Task>, description: String) {
    tasks.push(Task {
        description: description,
        completed: false,
    });
    println!("Tarefa adicionada com sucesso!");
}

fn mark_task_done(tasks: &mut Vec<Task>, task_id: usize) {
    let index = task_id - 1;

    if let Some(task) = tasks.get_mut(index) {
        task.completed = true;
    } else {
        println!("ERRO: ID inválido.");
    }
}
