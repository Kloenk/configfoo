use std::{error::Error, fs};

pub struct Config {
    man_file_path: String,
    tags_file_path: String,
    pub man_file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Use: mantags <manpage>");
        }
        Ok(Config { man_file_path: ".mantags".to_string(),
                    tags_file_path: "tags".to_string(),
                    man_file: args[1].clone(),
        })
    }
}
