#[path = "lib/task/mod.rs"] mod task;
#[path = "lib/storage/mod.rs"] mod storage;
use colored::*;
use storage::{FileStorage, Storage};
use std::string::*;

fn main() {
    let location = std::env::home_dir().map(|path| path.join("todo.json")).expect("No HOME dir found");
    let s = FileStorage::new(location.to_str().unwrap_or("/tmp/todo.json"));

    let tasks = s.get_tasks();
    if tasks.len() > 0 {
        let task_text = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

        for (i, task) in s.get_tasks().iter().filter(|t| t.get_body().to_lowercase().contains(&task_text)).enumerate() {
            let mut body = task.get_body().to_string();
            for hashtag in task.hashtags() {
                let hashtagged = format!("#{}", hashtag);
                body = body.replace(&hashtagged, &hashtagged.yellow().to_string());
            }
            println!("{}. {}", i+1, body);
        }
    } else {
        println!("Everything is done!");
    }

    
}