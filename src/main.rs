#![recursion_limit = "128"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate reqwest;

mod schema;
mod models;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;

fn main() {
    dotenv().ok();
    let connection = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    };


    let reader = BufReader::new(File::open("logfile19").unwrap());
    connection.execute("BEGIN TRANSACTION").expect("Failed to start transaction");
    for line in reader.lines().map(|l| l.expect("Failed to read lines from log file")) {
        // Annoyance: we can't just call .split(':') because place uses "::",
        // e.g. Dungeon 5 is D::5
        // So, next_real_delimiter checks for escaped delimiters and we advance in chunks based on that
        let mut entry = models::NewGame {
            gid: String::from("test"),
            xl: 0,
            score: 0
        };
        let mut slice = line.as_ref();
        let mut opt_name = None;
        let mut opt_start = None;
        // TODO figure out what to do with these expects (probably log and continue)
        while let Some(index) = next_real_delimiter(&slice) {
            let mut iter = slice[..index].split('=');
            let key = iter.next().expect("Corrupt data, missing key");
            let value = iter.next().expect("Corrupt data, missing value");
            slice = &slice[index+1..]; // TODO this could slice in the middle of a char boundary and panic if a key contains fancy unicode
            match key {
                "xl" => {
                    entry.xl = value.parse::<i64>().expect("Failed to parse xl");
                },
                "sc" => {
                    entry.score = value.parse::<i64>().expect("Failed to parse score");
                },
                "name" => {
                    opt_name = Some(value);
                },
                "start" => {
                    opt_start = Some(value);
                }
                _ => { /* Unknown or unused key TODO probably log it */ }
            }
        }
        if let (Some(name), Some(start)) = (opt_name, opt_start) {
            entry.gid = format!("{}{}{}", name, "cao", start);
            {
                use schema::games;
                diesel::insert(&entry).into(games::table).execute(&connection).expect("Error saving new game");
            }
        } else {
            // TODO log
            println!("Missing critical info, continuing");
        }
    }
    connection.execute("END TRANSACTION").expect("Failed to end transaction");
    println!("Parsing done");
}

fn next_real_delimiter(haystack: &str) -> Option<usize> {
    let mut offset = 0;
    loop {
        let substack = &haystack[offset..];
        if let Some(index) = substack.find(':') {
            if let Some(character) = substack.get(index+1..index+2) {
                if character == ":" {
                    // Escaped
                    offset += index + 2;
                    continue;
                } else {
                    return Some(offset + index);
                }
            } else {
                return Some(offset + index);
            }
        } else {
            return None;
        }
    }
}
