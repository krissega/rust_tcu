mod task;
mod storage;

use std::io::{self, Write};
use task::{Task, TaskStatus};
use storage::{save_tasks, load_tasks};

fn main() {
    // Carga las tareas guardadas o crea lista vacía
    let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());

    loop {
        println!("\n== Gestor de Tareas ==");
        println!("1) Listar tareas");
        println!("2) Agregar tarea");
        println!("3) Marcar tarea como completada");
        println!("4) Eliminar tarea");
        println!("5) Guardar y salir");

        print!("Elige una opción: ");
        io::stdout().flush().unwrap();

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();
        let opcion = opcion.trim();

        match opcion {
            "1" => listar_tareas(&tasks),
            "2" => agregar_tarea(&mut tasks),
            "3" => completar_tarea(&mut tasks),
            "4" => eliminar_tarea(&mut tasks),
            "5" => {
                save_tasks(&tasks).expect("Error guardando tareas");
                println!("¡Tareas guardadas! ¡Adiós!");
                break;
            }
            _ => println!("Opción no válida, intenta de nuevo."),
        }
    }
}

fn listar_tareas(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No hay tareas aún.");
    } else {
        println!("\nLista de tareas:");
        for (i, task) in tasks.iter().enumerate() {
            let estado = match task.status {
                TaskStatus::Pending => "Pendiente",
                TaskStatus::Done => "Completada",
            };
            println!("{}: [{}] {}", i + 1, estado, task.description);
        }
    }
}

fn agregar_tarea(tasks: &mut Vec<Task>) {
    println!("Escribe la descripción de la tarea:");
    let mut desc = String::new();
    io::stdin().read_line(&mut desc).unwrap();
    let desc = desc.trim();

    if desc.is_empty() {
        println!("La descripción no puede estar vacía.");
    } else {
        tasks.push(Task::new(desc.to_string()));
        println!("Tarea agregada.");
    }
}

fn completar_tarea(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No hay tareas para completar.");
        return;
    }

    listar_tareas(tasks);
    println!("Número de tarea a marcar como completada:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(num) = input.trim().parse::<usize>() {
        if num > 0 && num <= tasks.len() {
            tasks[num - 1].status = TaskStatus::Done;
            println!("Tarea marcada como completada.");
        } else {
            println!("Número fuera de rango.");
        }
    } else {
        println!("Entrada inválida.");
    }
}

fn eliminar_tarea(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No hay tareas para eliminar.");
        return;
    }

    listar_tareas(tasks);
    println!("Número de tarea a eliminar:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(num) = input.trim().parse::<usize>() {
        if num > 0 && num <= tasks.len() {
            tasks.remove(num - 1);
            println!("Tarea eliminada.");
        } else {
            println!("Número fuera de rango.");
        }
    } else {
        println!("Entrada inválida.");
    }
}
