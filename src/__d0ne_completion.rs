#[path = "lib/task/mod.rs"] mod task;
#[path = "lib/storage/mod.rs"] mod storage;
use storage::{FileStorage, Storage};

// complete -C __d0ne_completion d0ne

fn main() {
    let location = std::env::home_dir().map(|path| path.join("todo.json")).expect("No HOME dir found");
    let s = FileStorage::new(location.to_str().unwrap_or("/tmp/todo.json"));

    let line = std::env::var("COMP_LINE").expect("No COMP_LINE");

    let text = line.split(" ").skip(1).map(|x| x.to_string().replace("_", " ")).collect::<Vec<String>>().join(" "); // 
    for task in s.get_tasks().iter() {
        let body = task.get_body();
        if body.contains(&text) {
            let good_body = body.replace(" ", "_");
            println!("{}", good_body);
        }
    }
}