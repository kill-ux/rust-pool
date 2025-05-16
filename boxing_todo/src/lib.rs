mod err;
use std::fs::File;
use std::io::Read;

use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        match json::parse(&content) {
            Ok(res) => {
                let mut empty = true;
                let todo = TodoList {
                    title: res["title"].to_string(),
                    tasks: res["tasks"]
                        .members()
                        .map(|ts| {
                            empty = false;
                            Task {
                                id: ts["id"].as_u32().unwrap(),
                                description: ts["description"].to_string(),
                                level: ts["level"].as_u32().unwrap(),
                            }
                        })
                        .collect(),
                };
                if empty {
                    return Err(Box::new(err::ParseErr::Empty));
                }
                Ok(todo)
            }
            Err(err) => Err(Box::new(err::ParseErr::Malformed(Box::new(err)))),
        }
    }
}
