#[path = "lib/task/mod.rs"] mod task;
#[path = "lib/storage/mod.rs"] mod storage;
use storage::{FileStorage, Storage};

fn main() {
    let s = FileStorage::new("~/todos.json");

    let task_text = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

    for (i, task) in s.get_tasks().iter().filter(|t| t.get_body().to_lowercase().contains(&task_text)).enumerate() {
        println!("{}. {}", i+1, task.get_body());
    }
}