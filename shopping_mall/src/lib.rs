use std::collections::HashMap;

#[inline]
fn coerce_map<V>(m: HashMap<impl Into<String>, V>) -> HashMap<String, V> {
    m.into_iter().map(|(k, v)| (k.into(), v)).collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mall {
    pub name: String,
    pub guards: HashMap<String, Guard>,
    pub floors: HashMap<String, Floor>,
}

impl Mall {
    pub fn new(
        name: impl Into<String>,
        guards: HashMap<impl Into<String>, Guard>,
        floors: HashMap<impl Into<String>, Floor>,
    ) -> Self {
        Self {
            name: name.into(),
            guards: coerce_map(guards),
            floors: coerce_map(floors),
        }
    }

    pub fn change_name(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
    }

    pub fn hire_guard(&mut self, name: impl Into<String>, guard: Guard) {
        self.guards.insert(name.into(), guard);
    }

    pub fn fire_guard(&mut self, name: impl Into<String>) {
        self.guards.remove(&name.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Guard {
    pub age: u32,
    pub years_experience: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Floor {
    pub stores: HashMap<String, Store>,
    pub size_limit: u64,
}

impl Floor {
    pub fn new(stores: HashMap<impl Into<String>, Store>, size_limit: u64) -> Self {
        Self {
            stores: coerce_map(stores),
            size_limit,
        }
    }

    pub fn replace_store(&mut self, store: impl Into<String>, with: Store) {
        self.stores.entry(store.into()).and_modify(|v| *v = with);
    }

    pub fn add_store(&mut self, name: impl Into<String>, store: Store) -> Result<(), ()> {
        let has_space = self.size_limit
            >= self.stores.values().map(|s| s.square_meters).sum::<u64>() + store.square_meters;

        if has_space {
            self.stores.insert(name.into(), store);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn remove_store(&mut self, name: impl Into<String>) {
        self.stores.remove(&name.into());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub employees: HashMap<String, Employee>,
    pub square_meters: u64,
}

impl Store {
    pub fn new(employees: HashMap<impl Into<String>, Employee>, square_meters: u64) -> Self {
        Self {
            employees: coerce_map(employees),
            square_meters,
        }
    }

    pub fn hire_employee(&mut self, name: impl Into<String>, employee: Employee) {
        self.employees.insert(name.into(), employee);
    }

    pub fn fire_employee(&mut self, name: impl Into<String>) {
        self.employees.remove(&name.into());
    }

    pub fn expand(&mut self, by: u64) {
        self.square_meters += by;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Employee {
    pub age: u32,
    // The employee works from `working_hours.0` to `working_hours.1`
    pub working_hours: (u32, u32),
    pub salary: f64,
}

impl Employee {
    pub fn birthday(&mut self) {
        self.age += 1;
    }

    pub fn change_workload(&mut self, from: u32, to: u32) {
        self.working_hours = (from, to);
    }

    pub fn raise(&mut self, amount: f64) {
        self.salary += amount;
    }

    pub fn cut(&mut self, amount: f64) {
        self.salary -= amount;
    }
}
pub fn biggest_store(mall: &Mall) -> Option<(String, Store)> {
    let mut max_size = 0;
    let mut result = None;
    
    for (_floor_name, floor) in &mall.floors {
       for (store_name, store) in &floor.stores {
            if store.square_meters > max_size {
                max_size = store.square_meters;
                result = Some((store_name.clone(), store.clone()));
            }
       }
    }
    
    result
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(String,Employee)> {
    let mut result : Vec<(String,Employee)> = Vec::new();
    let mut biggest_salary_alt_the_shop : f64 = 0.0;
    for (_floor_name, floor_content) in &mall.floors {
        for (_store_name, store_content) in &floor_content.stores {
            for (_employee_name, employee_infos) in &store_content.employees {
                if employee_infos.salary >= biggest_salary_alt_the_shop {
                    biggest_salary_alt_the_shop = employee_infos.salary;
                   
                }
            }
        }
    }

    for (_floor_name, floor_content) in &mall.floors {
        for (_store_name, store_content) in &floor_content.stores {
            for (employee_name, employee_infos) in &store_content.employees {
                if employee_infos.salary == biggest_salary_alt_the_shop {
                    result.push((employee_name.clone(), *employee_infos));
                }
            }
        }
    }
    
    result
}

pub fn nbr_of_employees(mall : &Mall) -> usize {
    let mut nbr = mall.guards.len();
    for (_floor_name, floor) in &mall.floors {
        for (_, store_infos) in &floor.stores {
            nbr += store_infos.employees.len();
        }
    }
    nbr
}

pub fn check_for_securities(mall: &mut Mall, map: HashMap<String,Guard>) {
    let mut how_many = 0;
    for (_, floor_infos) in &mall.floors {  
        how_many += floor_infos.size_limit as usize/200;
    }
    if how_many > mall.guards.len() {
        for (guard_name, guard_infos) in &map {
            if how_many == mall.guards.len() {
                break;
            }
            mall.hire_guard(guard_name, *guard_infos);
        }
    }
    println!("{:?}",mall.guards.len());
}

pub fn cut_or_raise(mall: &mut Mall) {
    let mut raise: Vec<Employee> = Vec::new();
    let mut un_raise: Vec<Employee> = Vec::new();
    for (_floor_name, floor_content) in &mall.floors {
        for (_store_name, store_content) in &floor_content.stores {
            for (_employee_name, employee_infos) in &store_content.employees {
                if employee_infos.working_hours.1 - employee_infos.working_hours.0 > 10 {
                   raise.push(*employee_infos)
                }else {
                   un_raise.push(*employee_infos)
                }
            }
        }
    }
    for mut e in raise {
        e.raise((e.salary*10.0)/100.0);
    }
    for mut e in un_raise {
        e.cut((e.salary*10.0)/100.0);
    }
}



