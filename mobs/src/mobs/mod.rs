use std::collections::{HashMap, HashSet};

use crate::{Boss, Member, Role};

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(name.to_string(), Member::new(age));
    }

    pub fn attack(&mut self, mob: &mut Mob) {
        let mut last_member_a: String = String::new();
        let mut age = u32::MAX;

        let score_a = self
            .members
            .iter()
            .map(|(key, member)| {
                if age > member.age {
                    age = member.age;
                    last_member_a = key.clone();
                }
                match member.role {
                    Role::Associate => 1,
                    Role::Soldier => 2,
                    Role::Caporegime => 3,
                    Role::Underboss => 4,
                }
            })
            .sum::<u32>();

        let mut last_member_b: String = String::new();
        age = u32::MAX;

        let score_b: u32 = mob
            .members
            .iter()
            .map(|(key, member)| {
                if age > member.age {
                    age = member.age;
                    last_member_b = key.clone();
                }
                match member.role {
                    Role::Associate => 1,
                    Role::Soldier => 2,
                    Role::Caporegime => 3,
                    Role::Underboss => 4,
                }
            })
            .sum::<u32>();

        if score_a <= score_b {
            self.members.remove(&last_member_a);
            if self.members.len() == 0 {
                mob.wealth += self.wealth;
                self.wealth = 0;
                for key in self.cities.clone() {
                    self.cities.remove(&key);
                    mob.cities.insert(key);
                }
            }
        } else {
            mob.members.remove(&last_member_b);
            if mob.members.len() == 0 {
                self.wealth += mob.wealth;
                mob.wealth = 0;
                for key in mob.cities.clone() {
                    mob.cities.remove(&key);
                    self.cities.insert(key);
                }
            }
        }
    }

    pub fn steal(&mut self, mob_target: &mut Mob, value: u64) {
        if value > mob_target.wealth {
            self.wealth += mob_target.wealth;
            mob_target.wealth = 0;
        } else {
            self.wealth += value;
            mob_target.wealth = mob_target.wealth - value;
        }
    }

    pub fn conquer_city(&mut self, slice: &[&Mob], city: String) {
        let mut mov = false;
        for mob in slice {
            if mob.cities.contains(&city) {
                mov = true;
                break;
            }
        }
        if !mov {
            self.cities.insert(city);
        }
    }
}
