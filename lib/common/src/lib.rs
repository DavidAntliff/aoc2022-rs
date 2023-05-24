use std::io::{Read};
use color_eyre::eyre::Context;

pub fn load_input(filename: &str) -> color_eyre::Result<Vec<String>> {
    let mut file = std::fs::File::open(filename)
        .wrap_err(format!("opening {}", filename))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .wrap_err(format!("reading {}", filename))?;
    Ok(content
        .lines()
        .into_iter()
        .map(String::from)
        .collect())
}

#[cfg(test)]
mod tests {
    //use super::*;
}
