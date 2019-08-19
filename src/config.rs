pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str>{
        args.next();

        let query = match args.next() {
            Some(e) => e,
            None => return Err("Not found the query string"),
        };

        let filename = match args.next() {
            Some(e) => e,
            None => return Err("Not found the filename string"),
        };

        let case_sensitive = match args.next() {
            Some(e) => if e == "y".as_ref() {
                true
            } else { false }

            None => false
        };
        Ok(Config{query, filename, case_sensitive})
    }
}