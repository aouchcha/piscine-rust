mod mall;

use std::collections::HashMap;

// use mall::*;
use nn::*;


fn main() {
    let mut mall = Mall::new(
        "La Vie Funchal",
        [
            (
                "John Oliver",
                Guard {
                    age: 34,
                    years_experience: 7,
                },
            ),
            (
                "Bob Schumacher",
                Guard {
                    age: 53,
                    years_experience: 15,
                },
            ),
        ]
        .into(),
        [
            (
                "Ground Floor",
                Floor::new(
                    [
                        (
                            "Footzo",
                            Store::new(
                                [
                                    (
                                        "Finbar Haines",
                                        Employee {
                                            age: 36,
                                            working_hours: (9, 14),
                                            salary: 650.88,
                                        },
                                    ),
                                    (
                                        "Sienna-Rose Penn",
                                        Employee {
                                            age: 26,
                                            working_hours: (9, 22),
                                            salary: 1000.43,
                                        },
                                    ),
                                ]
                                .into(),
                                50,
                            ),
                        ),
                        (
                            "Swashion",
                            Store::new(
                                [
                                    (
                                        "Abdallah Stafford",
                                        Employee {
                                            age: 54,
                                            working_hours: (8, 22),
                                            salary: 1234.21,
                                        },
                                    ),
                                    (
                                        "Marian Snyder",
                                        Employee {
                                            age: 21,
                                            working_hours: (8, 14),
                                            salary: 831.9,
                                        },
                                    ),
                                ]
                                .into(),
                                43,
                            ),
                        ),
                    ]
                    .into(),
                    300,
                ),
            ),
            (
                "Supermarket",
                Floor::new(
                    [(
                        "Pretail",
                        Store::new(
                            [
                                (
                                    "Yara Wickens",
                                    Employee {
                                        age: 39,
                                        working_hours: (9, 14),
                                        salary: 853.42,
                                    },
                                ),
                                (
                                    "Indiana Baxter",
                                    Employee {
                                        age: 33,
                                        working_hours: (13, 20),
                                        salary: 991.71,
                                    },
                                ),
                                (
                                    "Jadine Page",
                                    Employee {
                                        age: 48,
                                        working_hours: (13, 20),
                                        salary: 743.21,
                                    },
                                ),
                                (
                                    "Tyler Hunt",
                                    Employee {
                                        age: 63,
                                        working_hours: (13, 20),
                                        salary: 668.25,
                                    },
                                ),
                                (
                                    "Mohsin Mcgee",
                                    Employee {
                                        age: 30,
                                        working_hours: (19, 24),
                                        salary: 703.83,
                                    },
                                ),
                                (
                                    "Antoine Goulding",
                                    Employee {
                                        age: 45,
                                        working_hours: (19, 24),
                                        salary: 697.12,
                                    },
                                ),
                                (
                                    "Mark Barnard",
                                    Employee {
                                        age: 53,
                                        working_hours: (19, 24),
                                        salary: 788.81,
                                    },
                                ),
                            ]
                            .into(),
                            950,
                        ),
                    )]
                    .into(),
                    1000,
                ),
            ),
        ]
        .into(),
    );

    // returns the biggest store
    println!("Biggest store: {:#?}", biggest_store(&mall));

    // returns the list with the highest paid employees
    println!("Highest paid employee: {:#?}", highest_paid_employee(&mall));

    // returns the number of employees
    println!("Number of employees: {}", nbr_of_employees(&mall));

    // checks if it is needed to add securities
    check_for_securities(
        &mut mall,
        [
            (
                "Peter Solomons",
                Guard {
                    age: 45,
                    years_experience: 20,
                },
            ),
            (
                "William Charles",
                Guard {
                    age: 32,
                    years_experience: 10,
                },
            ),
            (
                "Leonardo Changretta",
                Guard {
                    age: 23,
                    years_experience: 0,
                },
            ),
            (
                "Vlad Levi",
                Guard {
                    age: 38,
                    years_experience: 8,
                },
            ),
            (
                "Faruk Berkai",
                Guard {
                    age: 40,
                    years_experience: 15,
                },
            ),
            (
                "Christopher Smith",
                Guard {
                    age: 35,
                    years_experience: 9,
                },
            ),
            (
                "Jason Mackie",
                Guard {
                    age: 26,
                    years_experience: 2,
                },
            ),
            (
                "Kenzie Mair",
                Guard {
                    age: 34,
                    years_experience: 8,
                },
            ),
            (
                "Bentley Larson",
                Guard {
                    age: 33,
                    years_experience: 10,
                },
            ),
            (
                "Ray Storey",
                Guard {
                    age: 37,
                    years_experience: 12,
                },
            ),
        ]
        .map(|(n, d)| (n.to_owned(), d))
        .into(),
    );

    // raises or cuts the salary of every employee
    cut_or_raise(&mut mall);

    println!("{:#?}", mall);
}


fn employees(mall: &mut Mall) -> HashMap<String, &mut Employee> {
    mall.floors
        .values_mut()
        .flat_map(|f| {
            f.stores
                .values_mut()
                .flat_map(|s| s.employees.iter_mut().map(|(k, v)| (k.clone(), v)))
        })
        .collect::<HashMap<_, _>>()
}

#[test]
fn biggest_store_tests() {
    let mut mall = mall();

    assert!(matches!(
        biggest_store(&mall),
        (
            name,
            Store {
                square_meters: 950,
                ..
            }
        ) if name == "Pretail"
    ));

    mall.floors
        .get_mut("Supermarket")
        .unwrap()
        .remove_store("Pretail");

    assert!(matches!(
        biggest_store(&mall),
        (
            name,
            Store {
                square_meters: 60,
                ..
            }
        ) if name == "PizBite"
    ));
}

#[test]
fn highest_paid_test() {
    let mut mall = mall();

    let highest_paid = highest_paid_employee(&mall);

    assert!(matches!(
        highest_paid[..],
        [(name, Employee { age: 54, .. })] if name == "Abdallah Stafford"
    ));

    let highest_salary = highest_paid[0].1.salary;

    let mut employees = employees(&mut mall);

    let another_employee = employees.get_mut("Finbar Haines").unwrap();

    another_employee.raise(highest_salary - another_employee.salary);

    let highest_paid = highest_paid_employee(&mall);

    assert_eq!(2, highest_paid.len());
    assert!(highest_paid
        .windows(2)
        .all(|w| w[0] != w[1] && w[0].1.salary == w[1].1.salary));
    assert!(highest_paid.into_iter().all(
        |v| matches!(v, (n, Employee { age: 54, .. }) if n == "Abdallah Stafford")
            | matches!(v, (n, Employee { age: 36, .. }) if n == "Finbar Haines")
    ));
}

#[test]
fn nbr_of_employees_test() {
    let mut mall = mall();

    assert_eq!(36, nbr_of_employees(&mall));

    mall.floors
        .get_mut("Supermarket")
        .unwrap()
        .stores
        .get_mut("Pretail")
        .unwrap()
        .employees
        .drain();

    assert_eq!(22, nbr_of_employees(&mall));
}

#[test]
fn check_for_securities_test() {
    let mut mall = mall();

    assert_eq!(3, mall.guards.len());

    check_for_securities(
        &mut mall,
        [
            (
                "Peter Solomons",
                Guard {
                    age: 45,
                    years_experience: 20,
                },
            ),
            (
                "William Charles",
                Guard {
                    age: 32,
                    years_experience: 10,
                },
            ),
            (
                "Leonardo Changretta",
                Guard {
                    age: 23,
                    years_experience: 0,
                },
            ),
            (
                "Vlad Levi",
                Guard {
                    age: 38,
                    years_experience: 8,
                },
            ),
            (
                "Faruk Berkai",
                Guard {
                    age: 40,
                    years_experience: 15,
                },
            ),
            (
                "Chritopher Smith",
                Guard {
                    age: 35,
                    years_experience: 9,
                },
            ),
            (
                "Jason Mackie",
                Guard {
                    age: 26,
                    years_experience: 2,
                },
            ),
            (
                "Kenzie Mair",
                Guard {
                    age: 34,
                    years_experience: 8,
                },
            ),
            (
                "Bentley Larson",
                Guard {
                    age: 33,
                    years_experience: 10,
                },
            ),
            (
                "Ray Storey",
                Guard {
                    age: 37,
                    years_experience: 12,
                },
            ),
        ]
        .map(|(n, d)| (n.to_owned(), d))
        .into(),
    );

    assert_eq!(9, mall.guards.len());
}

#[test]
fn cut_or_raise_test() {
    let mut mall = mall();

    cut_or_raise(&mut mall);

    {
        let employees = employees(&mut mall);

        assert_eq!(585.792, employees.get("Finbar Haines").unwrap().salary);
        assert_eq!(1100.473, employees.get("Sienna-Rose Penn").unwrap().salary);
    }

    cut_or_raise(&mut mall);

    {
        let employees = employees(&mut mall);

        assert_eq!(527.2128, employees.get("Finbar Haines").unwrap().salary);
        assert_eq!(1210.5203, employees.get("Sienna-Rose Penn").unwrap().salary);
    }
}

pub fn mall() -> Mall {
    Mall::new(
        "La Vie Funchal",
        [
            (
                "John Oliver",
                Guard {
                    age: 34,
                    years_experience: 7,
                },
            ),
            (
                "Logan West",
                Guard {
                    age: 23,
                    years_experience: 2,
                },
            ),
            (
                "Bob Schumacher",
                Guard {
                    age: 53,
                    years_experience: 15,
                },
            ),
        ]
        .into(),
        [
            (
                "Ground Floor",
                Floor::new(
                    [
                        (
                            "Footzo",
                            Store::new(
                                [
                                    (
                                        "Finbar Haines",
                                        Employee {
                                            age: 36,
                                            working_hours: (9, 14),
                                            salary: 650.88,
                                        },
                                    ),
                                    (
                                        "Roksanna Rocha",
                                        Employee {
                                            age: 45,
                                            working_hours: (13, 22),
                                            salary: 772.,
                                        },
                                    ),
                                    (
                                        "Sienna-Rose Penn",
                                        Employee {
                                            age: 26,
                                            working_hours: (9, 22),
                                            salary: 1000.43,
                                        },
                                    ),
                                ]
                                .into(),
                                50,
                            ),
                        ),
                        (
                            "Swashion",
                            Store::new(
                                [
                                    (
                                        "Abdallah Stafford",
                                        Employee {
                                            age: 54,
                                            working_hours: (8, 22),
                                            salary: 1234.21,
                                        },
                                    ),
                                    (
                                        "Marian Snyder",
                                        Employee {
                                            age: 21,
                                            working_hours: (8, 14),
                                            salary: 831.9,
                                        },
                                    ),
                                    (
                                        "Amanda Mclean",
                                        Employee {
                                            age: 29,
                                            working_hours: (13, 22),
                                            salary: 1222.12,
                                        },
                                    ),
                                    (
                                        "Faizaan Castro",
                                        Employee {
                                            age: 32,
                                            working_hours: (11, 18),
                                            salary: 1106.43,
                                        },
                                    ),
                                ]
                                .into(),
                                43,
                            ),
                        ),
                    ]
                    .into(),
                    300,
                ),
            ),
            (
                "Food Floor",
                Floor::new(
                    [
                        (
                            "PizBite",
                            Store::new(
                                [
                                    (
                                        "Juniper Cannon",
                                        Employee {
                                            age: 21,
                                            working_hours: (16, 23),
                                            salary: 804.35,
                                        },
                                    ),
                                    (
                                        "Alena Simon",
                                        Employee {
                                            age: 28,
                                            working_hours: (9, 15),
                                            salary: 973.54,
                                        },
                                    ),
                                    (
                                        "Yasemin Collins",
                                        Employee {
                                            age: 29,
                                            working_hours: (9, 19),
                                            salary: 986.33,
                                        },
                                    ),
                                    (
                                        "Areeb Roberson",
                                        Employee {
                                            age: 54,
                                            working_hours: (9, 22),
                                            salary: 957.82,
                                        },
                                    ),
                                    (
                                        "Rocco Amin",
                                        Employee {
                                            age: 44,
                                            working_hours: (13, 23),
                                            salary: 689.21,
                                        },
                                    ),
                                ]
                                .into(),
                                60,
                            ),
                        ),
                        (
                            "Chillout Grill",
                            Store::new(
                                [
                                    (
                                        "Rhian Crowther",
                                        Employee {
                                            age: 45,
                                            working_hours: (9, 15),
                                            salary: 841.18,
                                        },
                                    ),
                                    (
                                        "Nikkita Steadman",
                                        Employee {
                                            age: 52,
                                            working_hours: (14, 22),
                                            salary: 858.61,
                                        },
                                    ),
                                    (
                                        "Reginald Poole",
                                        Employee {
                                            age: 32,
                                            working_hours: (9, 22),
                                            salary: 1197.64,
                                        },
                                    ),
                                    (
                                        "Minnie Bull",
                                        Employee {
                                            age: 54,
                                            working_hours: (14, 22),
                                            salary: 1229.73,
                                        },
                                    ),
                                ]
                                .into(),
                                50,
                            ),
                        ),
                        (
                            "Sumo Food",
                            Store::new(
                                [
                                    (
                                        "Chantelle Barajas",
                                        Employee {
                                            age: 20,
                                            working_hours: (8, 22),
                                            salary: 969.22,
                                        },
                                    ),
                                    (
                                        "Hywel Rudd",
                                        Employee {
                                            age: 49,
                                            working_hours: (12, 22),
                                            salary: 695.74,
                                        },
                                    ),
                                    (
                                        "Marianne Beasley",
                                        Employee {
                                            age: 55,
                                            working_hours: (8, 14),
                                            salary: 767.83,
                                        },
                                    ),
                                ]
                                .into(),
                                30,
                            ),
                        ),
                    ]
                    .into(),
                    500,
                ),
            ),
            (
                "Supermarket",
                Floor::new(
                    [(
                        "Pretail",
                        Store::new(
                            [
                                (
                                    "Amara Schaefer",
                                    Employee {
                                        age: 23,
                                        working_hours: (9, 14),
                                        salary: 796.21,
                                    },
                                ),
                                (
                                    "Yara Wickens",
                                    Employee {
                                        age: 39,
                                        working_hours: (9, 14),
                                        salary: 853.42,
                                    },
                                ),
                                (
                                    "Tomi Boyer",
                                    Employee {
                                        age: 64,
                                        working_hours: (9, 14),
                                        salary: 881.83,
                                    },
                                ),
                                (
                                    "Greta Dickson",
                                    Employee {
                                        age: 42,
                                        working_hours: (9, 14),
                                        salary: 775.1,
                                    },
                                ),
                                (
                                    "Caroline Finnegan",
                                    Employee {
                                        age: 41,
                                        working_hours: (9, 14),
                                        salary: 702.92,
                                    },
                                ),
                                (
                                    "Indiana Baxter",
                                    Employee {
                                        age: 33,
                                        working_hours: (13, 20),
                                        salary: 991.71,
                                    },
                                ),
                                (
                                    "Jadine Page",
                                    Employee {
                                        age: 48,
                                        working_hours: (13, 20),
                                        salary: 743.21,
                                    },
                                ),
                                (
                                    "Husna Ryan",
                                    Employee {
                                        age: 43,
                                        working_hours: (13, 20),
                                        salary: 655.75,
                                    },
                                ),
                                (
                                    "Tyler Hunt",
                                    Employee {
                                        age: 63,
                                        working_hours: (13, 20),
                                        salary: 668.25,
                                    },
                                ),
                                (
                                    "Dahlia Caldwell",
                                    Employee {
                                        age: 56,
                                        working_hours: (13, 20),
                                        salary: 781.38,
                                    },
                                ),
                                (
                                    "Chandler Mansell",
                                    Employee {
                                        age: 20,
                                        working_hours: (19, 24),
                                        salary: 656.75,
                                    },
                                ),
                                (
                                    "Mohsin Mcgee",
                                    Employee {
                                        age: 30,
                                        working_hours: (19, 24),
                                        salary: 703.83,
                                    },
                                ),
                                (
                                    "Antoine Goulding",
                                    Employee {
                                        age: 45,
                                        working_hours: (19, 24),
                                        salary: 697.12,
                                    },
                                ),
                                (
                                    "Mark Barnard",
                                    Employee {
                                        age: 53,
                                        working_hours: (19, 24),
                                        salary: 788.81,
                                    },
                                ),
                            ]
                            .into(),
                            950,
                        ),
                    )]
                    .into(),
                    1000,
                ),
            ),
        ]
        .into(),
    )
}