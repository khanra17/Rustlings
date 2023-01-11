#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}


impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.is_empty(){
            return Person::default()
        }

        let parts = s.split(",").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Person::default();
        }

        let name = parts[0].trim();
        if name.is_empty() {
            return Person::default();
        }

        let age_str = parts[1].trim();
        match age_str.parse::<usize>() {
            Ok(age) => Person { name: name.to_string(), age },
            Err(_) => Person::default(),
        }
    }
}

