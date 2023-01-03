use std::fmt::Error;

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.len() > 0 {
        Ok(format!("Hi! My name is {}", name))
    } else {
        // The error message should be: "`name` was empty; it must be nonempty."
        Err(String::from("`name` was empty; it must be nonempty."))
    }
}
