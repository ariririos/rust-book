pub fn search<'a>(
    query: &'a str,
    contents: &'a str
) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(
        contents
        .lines()
        .filter(move |line| line.contains(query))
    )
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    let query = query.to_lowercase();

    Box::new(
        contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
    )
}