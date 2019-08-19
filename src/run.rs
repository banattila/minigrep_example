use crate::config::Config;
use std::error::Error;
use std::fs;
use crate::searches::{sensitive_search, insensitive_search};

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        sensitive_search(&config.query, &contents)
    } else {
        insensitive_search(&config.query, &contents)
    };

    result.iter()
        .for_each(|line| println!("{}", line));

    Ok(())
}