#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

type TodoList = Mutex<Vec<Todo>>;

#[get("/")]
fn index() -> &'static str {
    "Todo API is running!"
}

#[get("/todos")]
fn get_todos(todos: &State<TodoList>) -> Json<Vec<Todo>> {
    let todos = todos.lock().expect("lock todos");
    Json(todos.clone())
}

#[post("/todos", data = "<todo>")]
fn create_todo(todos: &State<TodoList>, todo: Json<Todo>) -> Json<Todo> {
    let mut todos = todos.lock().expect("lock todos");
    let mut new_todo = todo.into_inner();
    new_todo.id = todos.len() + 1;
    todos.push(new_todo.clone());
    Json(new_todo)
}

#[put("/todos/<id>", data = "<todo>")]
fn update_todo(todos: &State<TodoList>, id: usize, todo: Json<Todo>) -> Option<Json<Todo>> {
    let mut todos = todos.lock().expect("lock todos");
    if let Some(existing) = todos.iter_mut().find(|t| t.id == id) {
        existing.title = todo.title.clone();
        existing.completed = todo.completed;
        Some(Json(existing.clone()))
    } else {
        None
    }
}

#[delete("/todos/<id>")]
fn delete_todo(todos: &State<TodoList>, id: usize) -> Option<Json<()>> {
    let mut todos = todos.lock().expect("lock todos");
    let index = todos.iter().position(|t| t.id == id)?;
    todos.remove(index);
    Some(Json(()))
}

#[launch]
fn rocket() -> _ {
    let initial_todos = vec![
        Todo {
            id: 1,
            title: "Learn Rust".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Build a Todo App".to_string(),
            completed: false,
        },
    ];

    rocket::build()
        .manage(Mutex::new(initial_todos))
        .mount("/api", routes![index, get_todos, create_todo, update_todo, delete_todo])
        .mount("/", rocket::fs::FileServer::from("static"))
}