#![recursion_limit = "128"]

extern crate flame;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

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
use std::collections::HashMap;

const LOG_CHUNK_SIZE: u64 = 1000;

#[derive(Deserialize)]
struct RequestResult {
    status: u16,
    message: String,
    offset: u64,
    next_offset: u64,
    results: Vec<GameResult>
}

#[derive(Deserialize)]
struct GameResult {
    id: u64,
    #[serde(rename="type")]
    game_type: String,
    data: GameData,
    time: u64,
    src_abbr: String
}

#[derive(Deserialize)]
struct GameData {
    v: String,
    lv: String,
    name: String,
    uid: String,
    race: String,
    cls: String,
    #[serde(rename="char")]
    start_code: String,
    xl: String,
    sk: String,
    sklev: String,
    title: String,
    place: String,
    br: String,
    lvl: String,
    mhp: String,
    mmhp: String,
    #[serde(rename="str")]
    strength: String,
    int: String,
    dex: String,
    start: String,
    dur: String,
    turn: String,
    sc: String,
    ktyp: String,
    killer: Option<String>,
    kaux: Option<String>,
    end: String,
    tmsg: String,
    urune: Option<String>,
    vmsg: Option<String>,
    potionsused: Option<String>,
    scrollsused: Option<String>,
    dam: Option<String>,
    tdam: Option<String>,
    sdam: Option<String>,
}

fn to_model(data: GameData, src_abbr: &str) -> Result<models::NewGame, std::num::ParseIntError> {
    let dam = if let Some(dam_str) = data.dam {
        dam_str.parse::<i64>()?
    } else {
        0
    };
    Ok(models::NewGame {
        gid: { format!("{}{}{}", data.name, src_abbr, data.start) },
        xl: {
            data.xl.parse::<i64>()?
        },
        tmsg: data.tmsg.clone(),
        turn: {
            data.turn.parse::<i64>()?
        },
        score: {
            data.sc.parse::<i64>()?
        },
        start: data.start.clone(),
        end: data.end.clone(),
        potions_used: {
            if let Some(potions_used_str) = data.potionsused {
                potions_used_str.parse::<i64>()?
            } else {
                -1
            }
        },
        scrolls_used: {
            if let Some(scrolls_used_str) = data.scrollsused {
                scrolls_used_str.parse::<i64>()?
            } else {
                -1
            }
        },
        dam: {
            dam
        },
        dur: {
            data.dur.parse::<i64>()?
        },
        runes: {
            if let Some(runes_str) = data.urune {
                runes_str.parse::<i64>()?
            } else {
                0
            }
        },
        tdam: {
            if let Some(tdam_str) = data.tdam {
                tdam_str.parse::<i64>()?
            } else {
                dam
            }
        },
        sdam: {
            if let Some(sdam_str) = data.sdam {
                sdam_str.parse::<i64>()?
            } else {
                dam
            }
        }
    })
}

fn main() {
    dotenv().ok();
    let connection = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    };

    let client = reqwest::Client::new();
    let mut offset = 0;
    let start_time = std::time::Instant::now();
    let mut stashed_games = 0;
    flame::start("loop");
    loop {
        flame::start("request and json");
        flame::start("request");
        let mut temp_response = client.get(&format!("http://crawlapi.mooo.com/event?offset={}&limit={}&type=game", offset, LOG_CHUNK_SIZE)).send().expect("Something");
        flame::end("request");
        flame::start("json");
        let response: RequestResult = temp_response.json().expect("Failed to deserialize JSON");
        flame::end("json");
        flame::end("request and json");
        if response.results.len() == 0 {
            // Done
            break;
        }
        connection.execute("BEGIN TRANSACTION").expect("Failed to start transaction");
        for game_result in response.results.into_iter() {
            flame::start("convert to model");
            if let Ok(game_model) = to_model(game_result.data, &game_result.src_abbr) {
                use schema::games;
                diesel::replace_into(games::table).values(&game_model).execute(&connection).expect("Error stashing new game");
            } else {
                println!("Failed to parse, skipping")
            }
            flame::end("convert to model");
        }
        flame::start("execute end transaction");
        connection.execute("END TRANSACTION").expect("Failed to end transaction");
        flame::end("execute end transaction");
        offset = response.next_offset;
        stashed_games += LOG_CHUNK_SIZE;
        if stashed_games % 10000 == 0 {
            println!("{} games parsed in {} secs", stashed_games, start_time.elapsed().as_secs());
            break;
        }
    }
    flame::end("loop");
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}


/*
fn parse_physical_log(file_name: &str, connection: &diesel::SqliteConnection) {
    let reader = BufReader::new(File::open(file_name).unwrap());
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
                diesel::insert(&entry).into(games::table).execute(connection).expect("Error saving new game");
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
} */
