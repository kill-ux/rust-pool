mod mall;
use std::iter::Map;

pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    mall.floors
        .values()
        .flat_map(|a| a.stores.clone())
        .max_by(|a, b| a.1.square_meters.cmp(&b.1.square_meters))
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, Employee)> {
    let mut res: Vec<(&str, Employee)> = Vec::new();
    let mut last_salary = 0.;
    mall.floors
        .values()
        .flat_map(|a| a.stores.iter())
        .flat_map(|a| a.1.employees.iter())
        .for_each(|a| {
            if a.1.salary == last_salary {
                res.push((a.0, a.1.clone()));
            } else if a.1.salary > last_salary {
                res.clear();
                res.push((a.0, a.1.clone()));
                last_salary = a.1.salary;
            }
        });
    res
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.guards.len()
        + mall
            .floors
            .values()
            .flat_map(|a| a.stores.clone())
            .flat_map(|a| a.1.employees.clone())
            .collect::<Vec<_>>()
            .len()
}

pub fn check_for_securities(mall: &mut Mall, mut map: Vec<(String, Guard)>) {
    let mut size = 0;
    mall.floors.values().for_each(|f| size += f.size_limit);
    let times = size / 200;
    let g = mall.guards.len() as u64;
    for _ in g..times {
        let a = map.pop().unwrap();
        mall.hire_guard(a.0, a.1);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    mall.floors
        .values_mut()
        .flat_map(|a| a.stores.values_mut())
        .flat_map(|a| a.employees.values_mut())
        .for_each(|f| {
            let r = f.working_hours.1 - f.working_hours.0;
            if r > 10 {
                f.raise(0.1 * f.salary);
            } else {
                f.cut(0.1 * f.salary);
            }
        });
}
