use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        let query = args[2].clone();

        Config { filename, query }
    }

    pub fn run(config: Config) {
        let _file_content = fs::read_to_string(config.filename).expect("Can't read the file");
        println!("sample -> {} \n", &_file_content[..100]);

        let found = Config::search(config.query.as_str(), _file_content.as_str());

        for element in found {
            println!("found -> {}", element)
        }

    }

    //lifetimes
    pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
        let mut result_line = Vec::new();

        for line in content.lines() {
            if line.contains(query) {
                result_line.push(line);
            }
        }

        result_line
    }
}
