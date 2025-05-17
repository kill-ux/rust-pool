pub mod mall;
pub use mall::*;
use std::collections::HashMap;

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

pub fn highest_paid_employee(m: &Mall) -> Vec<(&str, Employee)> {
    let mut vec: Vec<(&str, Employee)> = Vec::new();
    let mut salary = 0.0;

    for (_, f) in &m.floors {
        for (_, s) in &f.stores {
            for (key, e) in &s.employees {
                if e.salary == salary {
                    vec.push((key, e.clone()));
                } else if e.salary > salary {
                    salary = e.salary;
                    vec = vec![(key, e.clone())]
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
    let mut number = 0;
    m.floors.values().for_each(|flr| {
        number += flr.size_limit;
    });

    let num = number / 200;
    let l = m.guards.len() as u64;
    if l < num {
        for _ in 0..num - l {
            let (key, value) = map.pop().unwrap();
            m.guards.insert(key, value);
        }
    }
}

pub fn cut_or_raise(m: &mut Mall) {
    m.floors.values_mut().for_each(|flr| {
        flr.stores.values_mut().for_each(|st| {
            st.employees.values_mut().for_each(|emp| {
                if emp.working_hours.1 - emp.working_hours.0 >= 10 {
                    emp.raise(emp.salary * 0.1);
                } else {
                    emp.cut(emp.salary * 0.1);
                }
            });
        });
    });
}
