use std::env;
use std::process;
use testgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    let config_build_result = Config::build(&args);

    let config = match config_build_result {
        Ok(config) => config,
        Err(error) => {
            println!("Problem parsing arguments: {error}");
            process::exit(1);
        }
    };

    println!("Searching for {}", config.query);

    if let Err(e) = testgrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
