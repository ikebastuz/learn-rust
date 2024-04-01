pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did't get a file path"),
        };

        Ok(Config { file_path })
    }
}
