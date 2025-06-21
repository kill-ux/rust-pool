#[derive(Debug, PartialEq)]
pub struct Member {
    pub role: Role,
    pub age: u32,
}

impl Member {
    pub fn new(age: u32) -> Self {
        Self {
            role: Role::Associate,
            age,
        }
    }

    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Associate => Role::Soldier,
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Underboss => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

