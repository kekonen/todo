// #[path = "../task/mod.rs"] mod task;
use std::io::prelude::*;
use std::fs::OpenOptions;

use std::cell::RefCell;


use crate::task::Task;

// use std::fmt;

// #[derive(Debug)]
// struct StorageError {
//     details: String
// }

// impl StorageError {
//     fn new(msg: &str) -> StorageError {
//         StorageError{details: msg.to_string()}
//     }
// }

// impl fmt::Display for StorageError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,"{}",self.details)
//     }
// }

// impl Error for StorageError {
//     fn description(&self) -> &str {
//         &self.details
//     }
// }
use std::error::Error;
// use std::fs::File;
// use std::io::prelude::*;


pub trait Storage {
    fn add(&self, t: Task) -> Result<(), Box<dyn Error>>;
    fn drop(&self, id: &str) -> Result<(), Box<dyn Error>>;
    fn get_tasks(&self) -> Vec<Task>;
    // fn write(&self) -> Result<(), ()>;
    // fn read(&self) -> Result<(), ()>;
}

pub struct FileStorage {
    path: String,
    tasks: RefCell<Vec<Task>>,
}

impl FileStorage {
    pub fn new(path: &str) -> Self {
        
        let s = Self { path: path.to_string(), tasks: RefCell::new(vec![])};
        if let Err(e) = s.read() {
            eprintln!("Failed to read file {}: {:?}", s.path, e);
        }
        s
    }

    pub fn read(&self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)?;
            
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        if contents.len() > 0 {
            let tasks: std::vec::Vec<crate::task::Task> = serde_json::from_str(&contents)?;// contents.split("\n").map(|line| Task::new(line, None)).collect::<Vec<Task>>();
            *self.tasks.borrow_mut() = tasks;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&self.path)?;

        let tasks = &*self.tasks.borrow();
        let contents = serde_json::to_string_pretty(&tasks)?;// tasks.iter().map(|t| t.get_body().to_string()).collect::<Vec<String>>().join("\n");
        file.write_all(&contents.into_bytes())?;
        Ok(())

    }
}


impl Storage for FileStorage {
    fn add(&self, t: Task) -> Result<(), Box<dyn Error>> {
        let do_update = {
            let found = (*self.tasks.borrow_mut()).iter().any(|search_task| &t == search_task);
            if !found {
                (*self.tasks.borrow_mut()).push(t);
            }
            !found
        };
        
        if do_update {
            self.write()?
        }
        Ok(())
    }

    fn drop(&self, id: &str) -> Result<(), Box<dyn Error>> {
        {
            (*self.tasks.borrow_mut()).retain(|t| t.get_id() != id);
        }
        self.write()?;
        Ok(())
    }

    fn get_tasks(&self) -> Vec<Task> {
        (self.tasks.borrow()).clone()
    }
}