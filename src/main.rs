use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    //let contents = fs::read_to_string(file_path).expect("file read error");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        //let query = args[1].clone();
        //let file_path = args[2].clone();
        let query = &args[1];
        let file_path = &args[2];
        // vs clone ? 
        Config {query: query.to_string(),  file_path: file_path.to_string()}
    }
}


