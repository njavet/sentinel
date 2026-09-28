use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = run(config); {
        println!("application error: {e}");
        process::exit(1);
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>>  {
    let contents = fs::read_to_string(config.file_path)?;
    Ok(())
}


struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        //let query = args[1].clone();
        //let file_path = args[2].clone();
        let query = &args[1];
        let file_path = &args[2];
        // vs clone ? 
        Ok(Config {query: query.to_string(),  file_path: file_path.to_string()})
    }
}


