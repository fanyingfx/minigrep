use std::env;
use std::fs;
use std::process;
fn main() {
    // println!("Hello, world!");
    let args: Vec<_> = env::args().collect();
    // let query = &args[1];
    // let file_path = &args[2];
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {file_path}", file_path = &config.file_path);

    let contents =
        fs::read_to_string(&config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
    // dbg!(args);
}
struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
// fn parse_config(args: &[String]) -> Config {
//     let query=args[1].clone();
//     let file_path=args[2].clone();
//     // (query,file_path)
//     Config{query,file_path}
// }
