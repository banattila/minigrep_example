pub fn sensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    content
        .lines()
        .filter(|line|
            line.contains(query)
        )
        .collect()
}

pub fn insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    content
        .lines()
        .filter(|line|
            line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}