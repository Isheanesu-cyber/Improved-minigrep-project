extern crate minigrep;
use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let configure_minigrep: Config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments ! : {:?}", err);
        process::exit(1);
    });
    println!("Searching for {:?} ", configure_minigrep.query);
    println!("Filename : {:?}", configure_minigrep.filename);
    match minigrep::run(configure_minigrep) {
        Err(error) => {
            eprintln!("Application error !, {:?}", error);
            process::exit(1);
        }
        Ok(_) => {}
    };
}
