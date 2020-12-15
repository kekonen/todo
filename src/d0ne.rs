#[path = "lib/task/mod.rs"] mod task;
#[path = "lib/storage/mod.rs"] mod storage;
use task::Task;
use storage::{FileStorage, Storage};

fn main() {
    let s = FileStorage::new("~/todos.json");

    let task_text = std::env::args().skip(1).map(|word| word.replace("_", " ")).collect::<Vec<String>>().join(" ");
    let t = Task::new(&task_text, None);

    s.drop(t.get_id()).expect("Couldnt drop");
}
