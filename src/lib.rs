use std::env;
use std::io;

#[cfg(test)]

mod tests {

    //use std::env;

    use super::*;
    //use crate::Config;
    #[test]
    fn it_works() {
        let four: u32 = 4;
        assert_eq!(4, four);
    }
    #[test]
    fn test_search_file_for_word() {
        // Test case 1: Word exists in the file contents
        let contents = "
Hello World\n
This is a test\n
World cup";
        let query_word = "World";
        let expected_result = vec!["Hello World", "World cup"];

        assert_eq!(search_file_for_word(query_word, contents), expected_result);

        // Test case 2: Word does not exist in the file contents
        let contents = "Hello world\nThis is a test";
        let query_word = "python";
        let expected_result: Vec<&str> = Vec::new();

        assert_eq!(search_file_for_word(query_word, contents), expected_result);

        // Test case 3: Multiple occurrences of the word
        let contents = "Hello world\nWorld cup\nThis is a test";
        let query_word = "world";
        let expected_result = vec!["Hello world", "World cup"];

        assert_eq!(search_file_for_word(query_word, contents), expected_result);

        // Test case 4: Empty file contents
        let contents = "";
        let query_word = "test";
        let expected_result: Vec<&str> = Vec::new();

        assert_eq!(search_file_for_word(query_word, contents), expected_result);
    }
}
pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't put query string !"),
        };
        let filename: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't put filename !"),
        };
        Ok(Config {
            query: query,
            filename: filename,
        })
    }
}
/// A `Result` containing the file contents as a `String`, or an `Error` if reading fails.
pub fn run(config: Config) -> Result<String, io::Error> {
    // Use std::fs::read_to_string for simpler file reading
    let content: String = std::fs::read_to_string(&config.filename)?;

    // Print the contents (use {} instead of {:?} for better formatting)
    //println!("Contents of {}: \n{}", config.filename, content);
    for line in search_file_for_word(&config.query, &content) {
        println!("{} ", line);
    }

    // Return the contents
    Ok(content)
}
//pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//    let mut file: File = File::open(config.filename)?;
//    let mut content: String = String::new();
//    file.read_to_string(&mut content)?;
//    println!("Write in {:?} \n", content);
//    Ok(())
//}

pub fn search_file_for_word<'a>(query_word: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query_word))
        .collect()
}
