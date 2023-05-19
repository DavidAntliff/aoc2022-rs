use std::io::{Read, Result};

pub fn load_input(filename: &str) -> Result<Vec<String>> {
    let mut file = std::fs::File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content
        .lines()
        .into_iter()
        .map(String::from)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
}
