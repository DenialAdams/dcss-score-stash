#![recursion_limit = "256"]
#![feature(custom_attribute)]

extern crate crawl_model;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate reqwest;

use std::io::{BufRead, BufReader};
use std::fs::File;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let connection = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    };

    /* // Ensure enum tables are initialized
    {
        // TODO this could be a macro
        // Species
        {
            connection
                .execute("BEGIN TRANSACTION")
                .expect("Failed to start transaction");
            for new_species in constants::SPECIES.iter() {
                use schema::species;
                // TODO this needs to be fixed to be a conditional update
                diesel::replace_into(species::table)
                    .values(new_species)
                    .execute(&connection)
                    .expect("Error saving species");
            }
            connection
                .execute("END TRANSACTION")
                .expect("Failed to end transaction");
        }
        // Background
        {
            connection
                .execute("BEGIN TRANSACTION")
                .expect("Failed to start transaction");
            for new_bg in constants::BACKGROUNDS.iter() {
                use schema::backgrounds;
                // TODO this needs to be fixed to be a conditional update
                diesel::replace_into(backgrounds::table)
                    .values(new_bg)
                    .execute(&connection)
                    .expect("Error saving species");
            }
            connection
                .execute("END TRANSACTION")
                .expect("Failed to end transaction");
        }
    } */


    let reader = BufReader::new(File::open("logfile").unwrap());
    // TODO: we should chunk transactions; there is a max size to sql statements
    connection
        .execute("BEGIN TRANSACTION")
        .expect("Failed to start transaction");
    for line in reader
        .lines()
        .map(|l| l.expect("Failed to read lines from log file"))
    {
        let mut slice = line.as_ref();

        // Stats
        let mut xl = 0; // TODO REQ
        let mut score = 0; // TODO REQ
        let mut turn = 0; // TODO REQ
        let mut potions_used = -1;
        let mut scrolls_used = -1;
        let mut dam = 0;
        let mut sdam = 0;
        let mut tdam = 0;
        let mut opt_species = None;
        let mut opt_background = None;
        let mut dur = 0; // TODO REQ
        let mut runes = 0;
        let mut opt_name = None;
        let mut opt_start = None;
        let mut opt_end = None;
        let mut tmsg = "";

        // TODO figure out what to do with these expects (probably log and continue)
        loop {
            let index = next_real_delimiter(&slice);
            let mut iter = slice[..index].split('=');
            let key = iter.next().expect("Corrupt data, missing key");
            let value = iter.next().expect("Corrupt data, missing value");
            match key {
                "xl" => {
                    xl = value.parse::<i64>().expect("Failed to parse xl");
                }
                "sc" => {
                    score = value.parse::<i64>().expect("Failed to parse score");
                }
                "turn" => {
                    turn = value.parse::<i64>().expect("Failed to parse turn");
                }
                "name" => {
                    opt_name = Some(value);
                }
                "start" => {
                    opt_start = Some(value);
                }
                "end" => {
                    opt_end = Some(value);
                }
                "potionsused" => {
                    potions_used = value.parse::<i64>().expect("Failed to parse potions used");
                }
                "scrollsused" => {
                    scrolls_used = value.parse::<i64>().expect("Failed to parse scrolls used")
                }
                "dam" => {
                    dam = value.parse::<i64>().expect("Failed to parse dam");
                }
                "tdam" => {
                    tdam = value.parse::<i64>().expect("Failed to parse tdam");
                }
                "sdam" => {
                    sdam = value.parse::<i64>().expect("Failed to parse sdam");
                }
                "tmsg" => {
                    tmsg = value;
                }
                "urune" => {
                    runes = value.parse::<i64>().expect("Failed to parse urune");
                }
                "dur" => {
                    dur = value.parse::<i64>().expect("Failed to parse dur");
                }
                "race" => {
                    opt_species = Some(value.parse::<crawl_model::data::Species>().expect(&format!("Failed to parse species {}", value)));
                },
                "cls" => {
                    opt_background = Some(value.parse::<crawl_model::data::Background>().expect(&format!("Failed to parse background {}", value)));
                }
                _ => { /* Unknown or unused key TODO probably log it */ }
            }
            if index == slice.len() {
                break;
            } else {
                slice = &slice[index + 1..];
            }
        }
        if let (Some(name), Some(start), Some(end), Some(bg), Some(species)) = (opt_name, opt_start, opt_end, opt_background, opt_species) {
            let entry = crawl_model::db_model::NewGame {
                gid: &format!("{}{}{}", name, "cao", start),
                name: name,
                species_id: species as i64,
                background_id: bg as i64,
                xl: xl,
                tmsg: tmsg,
                turn: turn,
                score: score,
                start: start,
                end: end,
                potions_used: potions_used,
                scrolls_used: scrolls_used,
                dam: dam,
                tdam: tdam,
                sdam: sdam,
                dur: dur,
                runes: runes,
            };
            {
                use crawl_model::db_schema::games;
                diesel::replace_into(games::table)
                    .values(&entry)
                    .execute(&connection)
                    .expect("Error saving new game");
            }
        } else {
            // TODO log
            println!("Missing critical info, continuing");
        }
    }
    connection
        .execute("END TRANSACTION")
        .expect("Failed to end transaction");
    println!("Parsing done");
}

// Annoyance: we can't just call .split(':') because place uses "::",
// e.g. Dungeon 5 is D::5
// So, next_real_delimiter checks for escaped delimiters and we advance in chunks based on that
fn next_real_delimiter(haystack: &str) -> usize {
    let mut offset = 0;
    loop {
        // TODO this could slice in the middle of a char boundary and panic if a key contains fancy unicode
        let substack = &haystack[offset..];
        if let Some(index) = substack.find(':') {
            if let Some(character) = substack.get(index + 1..index + 2) {
                if character == ":" {
                    // Escaped
                    offset += index + 2;
                    continue;
                } else {
                    return offset + index;
                }
            } else {
                return offset + index;
            }
        } else {
            // The last element is not delimited with a :,
            // but the end of the last element is a valid delimiter the way we use it
            return haystack.len();
        }
    }
}
