pub mod mall;
use std::{collections::HashMap};

pub use mall::*;

pub fn biggest_store(m: &Mall) -> (String, Store) {
    let mut store = Store {
        employees: HashMap::new(),
        square_meters: 0,
    };
    let mut k: String = String::new();
    for (_, f) in &m.floors {
        for (key, s) in &f.stores {
            if s.square_meters > store.square_meters {
                store = s.clone();
                k = key.clone();
            }
        }
    }

    (k, store)
}

pub fn highest_paid_employee(m: &Mall) -> Vec<(String, Employee)> {
    let mut vec: Vec<(String, Employee)> = Vec::new();
    let mut salary = 0.0;

    for (_, f) in &m.floors {
        for (_, s) in &f.stores {
            for (key, e) in &s.employees {
                if e.salary == salary {
                    vec.push((key.clone(), e.clone()));
                } else if e.salary > salary {
                    salary = e.salary;
                    vec = vec![(key.clone(), e.clone())]
                }
            }
        }
    }

    vec
}

pub fn nbr_of_employees(m: &Mall) -> usize {
    let mut num: usize = m.guards.len();
    for (_, f) in &m.floors {
        for (_, s) in &f.stores {
            num += s.employees.len();
        }
    }
    num
}

pub fn check_for_securities(m: &mut Mall, mut map: Vec<(String, Guard)>) {
    let num = m
        .floors
        .values()
        .map(|flr| flr.stores.values().map(|st| st.square_meters).sum::<u64>())
        .sum::<u64>()
        / 200;

    let l = m.guards.len() as u64;
    if l < num {
        for _ in 0..num - l {
            let (key, value) = map.pop().unwrap();
            m.guards.insert(key, value);
        }
    }
}

pub fn cut_or_raise(m: &mut Mall) {
    let _ = m.floors.values_mut().map(|flr| {
        flr.stores.values_mut().map(|st| {
            st.employees.values_mut().map(|emp| {
                if emp.working_hours.1 - emp.working_hours.0 >= 10 {
                    emp.salary + (emp.salary * 0.9)
                } else {
                    emp.salary - (emp.salary * 0.9)
                }
            })
        })
    });
}
