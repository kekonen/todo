#[path = "lib/task/mod.rs"] mod task;
#[path = "lib/storage/mod.rs"] mod storage;
use task::Task;
use storage::{FileStorage, Storage};

fn main() {
    let location = std::env::home_dir().map(|path| path.join("todo.json")).expect("No HOME dir found");
    let s = FileStorage::new(location.to_str().unwrap_or("/tmp/todo.json"));

    let task_text = std::env::args().skip(1).collect::<Vec<String>>().join(" ").trim().to_string();

    if task_text != "" && !task_text.contains("\\n") {
        let t = Task::new(&task_text, None);
        s.add(t).expect("Couldnt add");
    }
}
