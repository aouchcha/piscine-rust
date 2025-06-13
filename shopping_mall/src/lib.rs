pub mod mall;

pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(m: &Mall) -> (String, Store) {
    let mut result: Store = Store::new(HashMap::<String, Employee>::new(), 0 as u64);
    let mut name : String = String::new();
    for (_, floor_data) in &m.floors {
        for (store_name, store_data) in &floor_data.stores {
            if store_data.square_meters > result.square_meters {
                name = store_name.clone();
                result = store_data.clone();
            }
        }
    }
    (name,result)
}

pub fn highest_paid_employee(m: &Mall) -> Vec<(&str, Employee)> {
    let mut res : Vec<(&str, Employee)> = Vec::new();
    let mut temp = 0.0;
    for (_, floor_data) in &m.floors {
        for (_, store_data) in &floor_data.stores {
            for (name, employee_data) in &store_data.employees {
                if employee_data.salary > temp {
                    res = Vec::new();
                    res.push((name, *employee_data));
                    temp = employee_data.salary
                } else if employee_data.salary == temp {
                    res.push((name, *employee_data));
                }
            }
        }
    }
    res
}

pub fn nbr_of_employees(m: &Mall) -> usize {
    let mut res : usize = 0;
    for (_, _) in &m.guards {
        res += 1;
    }
    for (_, floor_data) in &m.floors {
        for (_, store_data) in &floor_data.stores {
            for (_, _) in &store_data.employees {
                res += 1;
            }
        }
    }
    res
}

pub fn check_for_securities(m: &mut Mall, guards: Vec<(String, Guard)>) {
    let nbr_act = m.guards.len();
    // println!("{:?}",nbr_act);
    let mut nbr_need: f64 = 0.0;
    for (_, floor_data) in &m.floors {
        // println!("{:?}",(floor_data.size_limit as f64 /200.0).ceil());
        nbr_need += (floor_data.size_limit as f64 /200.0).round()
    }
    println!("{:?}",nbr_need);
    for i in 0..(nbr_need as usize - nbr_act -1) {
        m.hire_guard(&guards[i].0, guards[i].1);
    }

}

pub fn cut_or_raise(m: &mut Mall) {
    for (_, floor_data) in &mut m.floors {
        for (_, store_data) in &mut floor_data.stores {
            for (_, employee_data) in &mut store_data.employees {
                if employee_data.working_hours.1 - employee_data.working_hours.0 < 10 {
                    employee_data.cut(employee_data.salary * 0.1);
                }else {
                    employee_data.raise(employee_data.salary * 0.1);
                }
            }
        }
    }
}
