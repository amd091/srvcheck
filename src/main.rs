use reqwest::StatusCode;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Collecting command line arguments
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Opening file and storing server URLs in a vector
    let file = File::open(file_path).unwrap();
    let servers: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    // Iterating through the vector of server URLs
    for server in servers {
        let response = reqwest::blocking::get(&server);
        // Checking the response status code and printing status
        match response {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    println!("{} is active", server);
                } else {
                    println!("{} is inactive", server);
                }
            }
            Err(_) => {
                println!("{} error reading response", server);
            }
        }
    }
}
