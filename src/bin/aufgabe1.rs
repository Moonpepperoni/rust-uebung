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

// Alle Fehler waren unterhalb dieser Zeile

fn main() {
    let people = vec![
        ("Michael Scott", 3499.99),
        ("Dwight Schrute", 2500.),
        ("Jim Halpert", 2501.),
        ("Pam Beesly", 1818.),
        ("Stanley Hudson", 2405.34),
    ];
    // employees muss als mut deklariert sein, damit wir einen weiteren Employee pushen können
    let mut employees: Vec<OfficeEmployee> = people
        .iter()
        .map(|(name, salary)| OfficeEmployee {
            name: String::from(*name),
            salary: *salary,
        })
        .collect();

    employees.push(OfficeEmployee {
        name: String::from("Jan L"),
        salary: 300.0,
    });

    println!("Welcome to the Office! Let's get to know everyone:\n");
    // Hier muss employees geborrowed werden, da sonst employees in der Schleife konsumiert wird
    for e in &employees {
        if let Some(fname) = first_name(&e.name) {
            println!("{}: Hi! Nice to meet you.", fname)
        }
    }
    println!("Michael: It's important to understand the friendship dynamics in an Office. Let me tell you about the people who work here:\n");
    // Hier wird zweimal der gleiche Vec mutable geborrowt. Man muss unten die Funktionssignatur anpassen und dann von &mut zu & wechseln
    let pairs = all_pairs(&employees);
    let relationships = rank_relationship(&pairs);

    for r in &relationships {
        println!("{}", r.to_string());
    }

    // Nachdem man unten die Funktion verbessert hat muss man auch hier borrowen
    if let Some(avg_salary) = calc_mean_salary(&employees) {
        println!("\nMichael: Just so you know, the average salary here is ${:.2}. But I am certain you will make a lot more.", avg_salary);
    }
    // Hier muss man Option erstmal entpacken, bevor man auf die Attribute zugreift
    if let Some(you) = employees.get(employees.len() - 1) {
        println!(
            "You are {} and you make ${:.2} right now without bonuses",
            you.name, you.salary
        );
    }
}

// Hier darf man kein Ownership vom String übernehmen und muss somit &str entgegennehmen
// Wenn man hier Ownership nimmt, dann stiehlt man den Wert aus der struct
fn first_name(name: &str) -> Option<&str> {
    match name.find(" ") {
        Some(pos) => Some(&name[..pos]),
        _ => None,
    }
}

// Hier muss man den Vec borrowen, da der Vec auch nach diesem Funktionsaufruf noch benutzt wird
fn calc_mean_salary(people: &Vec<OfficeEmployee>) -> Option<f64> {
    if people.len() == 0 {
        return None;
    }
    let s: f64 = people.iter().map(|p| p.salary).sum();
    Some(s / (people.len() as f64))
}


fn rank_relationship(pairs: &Vec<(&OfficeEmployee, &OfficeEmployee)>) -> Vec<Relationship> {
    let mut r = vec![];
    // Alle anderen sollten als Freunde abgespeichert werden
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
            // Hier fehlte der Match-Arm, der alle fehlenden Relationen umsetzt
            (_, _) => Relationship::Friends(p.name.clone(), o.name.clone()),
        };
        r.push(rel);
    }
    r
}

// Hier gab es nichts zu tun
fn all_pairs(
  p1: &Vec<OfficeEmployee>,
) -> Vec<(&OfficeEmployee, &OfficeEmployee)> {
  let mut pairs = vec![];
  for p in p1 {
      for o in p1 {
          if p != o && !pairs.iter().any(|(l, r)| *l == o && *r == p) {
              pairs.push((p, o));
          }
      }
  }
  return pairs;
}
