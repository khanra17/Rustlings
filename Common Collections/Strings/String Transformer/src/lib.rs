pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod transformer {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = vec![];
        for (mut string, command) in input {
            output.push(match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(times) => {
                    for _ in 0..times {
                        string.push_str("bar");
                    }
                    string
                }
            })
        }
        output
    }
}