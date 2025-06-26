#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        self.grade = Some(Box::new(Worker {
            role,
            name,
            next: self.grade.take(),
        }));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.take() {
            Some(gride) => {
                self.grade = gride.next;
                Some(gride.name)
            }
            None => None,
        }
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        match &self.grade {
            Some(gride) => Some((gride.name.to_string(), gride.role.to_string())),
            None => None,
        }
    }
}
