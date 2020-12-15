use regex::Regex;
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// use rand::Rng;
// const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
//     abcdefghijklmnopqrstuvwxyz\
//     0123456789"; // )(*&^%$#@!~

// fn random_string(size: usize) -> String {
//     let mut rng = rand::thread_rng();

//     (0..size)
//     .map(|_| {
//         let idx = rng.gen_range(0, CHARSET.len());
//         CHARSET[idx] as char
//     })
//     .collect::<String>()
// }

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)] // , Hash
pub struct Task {
    body: String,
    id: String,
}
impl Hash for Task {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.body.hash(state);
    }
}

fn captures_re_first(s: &str, r: Regex) -> Vec<String> {
    r.captures_iter(s).filter_map(|x| {
        x.get(1).map(|y| y.as_str().to_string())
    }).collect()
}

impl Task {
    pub fn new(body: &str, some_id: Option<&str>) -> Self {
        let id = if let Some(id) = some_id {
            id.to_string()
        } else {
            String::from("temp")//random_string(10)
        };
        let mut s = Self { body: body.to_string(), id: id };
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        s.id = hasher.finish().to_string();
        s
    }

    pub fn hashtags(&self,) -> Vec<String> {
        let hashtags_re = Regex::new(r"\#(\w+)").unwrap();
        captures_re_first(&self.body, hashtags_re)
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}