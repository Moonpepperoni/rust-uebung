#[derive(PartialEq)]
struct OfficeEmployee {
    name: String,
    salary: f64,
}

enum Relationship {
    Friends(String, String),
    Frenemies(String, String),
    Married(String, String),
}

impl ToString for Relationship {
    fn to_string(&self) -> String {
        match self {
            Self::Friends(p, o) => format!("{} is friends with {}", p, o),
            Self::Frenemies(p, o) => format!("{} loves and hates {}", p, o),
            Self::Married(p, o) => format!("{} is married to {}", p, o),
        }
    }
}

// Alle Fehler sind unterhalb dieser Zeile

// FIXME
fn main() {
    let people = vec![
        ("Michael Scott", 3499.99),
        ("Dwight Schrute", 2500.),
        ("Jim Halpert", 2501.),
        ("Pam Beesly", 1818.),
        ("Stanley Hudson", 2405.34),
    ];
    let employees: Vec<OfficeEmployee> = people
        .iter()
        .map(|(name, salary)| OfficeEmployee {
            name: String::from(*name),
            salary: *salary,
        })
        .collect();

    // TODO Erstelle einen neuen OfficeEmployee mit deinem eigenen Namen und deinem Wunschgehalt  
    employees.push(OfficeEmployee {
        name: String::from("Jan L"),
        salary: 300.0,
    });

    println!("Welcome to the Office! Let's get to know everyone:\n");
    for e in employees {
      if let Some(fname) = first_name(&e.name) {
        println!("{}: Hi! Nice to meet you.", fname)
      }
    }
    println!("Michael: It's important to understand the friendship dynamics in an Office. Let me tell you about the people who work here:\n");
    let pairs = all_pairs(&mut employees, &mut employees);
    let relationships = rank_relationship(&pairs);

    for r in relationships {
        println!("{}", r.to_string());
    }

    if let Some(avg_salary) = calc_mean_salary(employees) {
      println!("\nMichael: Just so you know, the average salary here is ${:.2}. But I am certain you will make a lot more.", avg_salary);
    }
    let you = employees.get(employees.len() - 1);
    println!("You are {} and you make ${:.2} right now without bonuses", you.name, you.salary);
}

// FIXME 
fn first_name(name: String) -> Option<&str> {
    match name.find(" ") {
        Some(pos) => Some(&name[..pos]),
        _ => None,
    }
}

// FIXME
fn calc_mean_salary(people: Vec<OfficeEmployee>) -> Option<f64> {
    if people.len() == 0 {
        return None;
    }
    let s: f64 = people.iter().map(|p| p.salary).sum();
    Some(s / (people.len() as f64))
}

// FIXME
fn all_pairs<'a>(
    p1: &'a mut Vec<OfficeEmployee>,
    p2: &'a mut Vec<OfficeEmployee>,
) -> Vec<(&'a mut OfficeEmployee, &'a mut OfficeEmployee)> {
    let mut pairs = vec![];
    for p in p1 {
        for o in p2 {
            if p != o && !pairs.iter().any(|(l, r)| *l == o && *r == p) {
                pairs.push((p, o));
            }
        }
    }
    return pairs;
}

// FIXME
fn rank_relationship(pairs: &Vec<(&OfficeEmployee, &OfficeEmployee)>) -> Vec<Relationship> {
    let mut r = vec![];
    // Alle anderen sollten als Freunde abgespeichert werden;
    for (p, o) in pairs {
        let rel = match (p.name.as_str(), o.name.as_str()) {
            ("Jim Halpert", "Dwight Schrute") => {
                Relationship::Frenemies(p.name.clone(), o.name.clone())
            }
            ("Dwight Schrute", "Jim Halpert") => {
                Relationship::Frenemies(p.name.clone(), o.name.clone())
            }
            ("Pam Beesly", "Jim Halpert") => Relationship::Married(p.name.clone(), o.name.clone()),
            ("Jim Halpert", "Pam Beesly") => Relationship::Married(p.name.clone(), o.name.clone()),
        };
        r.push(rel);
    }
    r
}
