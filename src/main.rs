fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = configfoo::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(2);
    });

    println!("{}", config.man_file);
}
