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
extern crate serde_json;

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
use std::io::Read;

const LOG_CHUNK_SIZE: u64 = 1000;

#[derive(Deserialize)]
struct RequestResult<'a> {
    status: u16,
    message: &'a str,
    offset: u64,
    next_offset: u64,
    results: Vec<GameResult<'a>>
}

#[derive(Deserialize)]
struct GameResult<'a> {
    id: u64,
    #[serde(rename="type")]
    game_type: &'a str,
    data: GameData<'a>,
    time: u64,
    src_abbr: &'a str
}

#[derive(Deserialize)]
struct GameData<'a> {
    v: &'a str,
    lv: &'a str,
    name: &'a str,
    uid: &'a str,
    race: &'a str,
    cls: &'a str,
    #[serde(rename="char")]
    start_code: &'a str,
    xl: &'a str,
    sk: &'a str,
    sklev: &'a str,
    title: &'a str,
    place: &'a str,
    br: &'a str,
    lvl: &'a str,
    mhp: &'a str,
    mmhp: &'a str,
    #[serde(rename="str")]
    strength: &'a str,
    int: &'a str,
    dex: &'a str,
    start: &'a str,
    dur: &'a str,
    turn: &'a str,
    sc: &'a str,
    ktyp: &'a str,
    killer: Option<&'a str>,
    kaux: Option<&'a str>,
    end: &'a str,
    tmsg: &'a str,
    urune: Option<&'a str>,
    vmsg: Option<&'a str>,
    potionsused: Option<&'a str>,
    scrollsused: Option<&'a str>,
    dam: Option<&'a str>,
    tdam: Option<&'a str>,
    sdam: Option<&'a str>,
}

fn to_model<'a>(data: GameData<'a>, src_abbr: &str) -> Result<models::NewGame<'a>, std::num::ParseIntError> {
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
        tmsg: data.tmsg,
        turn: {
            data.turn.parse::<i64>()?
        },
        score: {
            data.sc.parse::<i64>()?
        },
        start: data.start,
        end: data.end,
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
    let mut buf = Vec::with_capacity(1046528);
    flame::start("loop");
    loop {
        {
            flame::start("request and json");
            flame::start("request");
            //let mut temp_response = client.get(&format!("http://crawlapi.mooo.com/event?offset={}&limit={}&type=game", offset, LOG_CHUNK_SIZE)).send().expect("Something");
            flame::end("request");
            flame::start("json");
            flame::start("fill buffer");
            let mut file = std::fs::File::open("/home/brick/new_dcss/event.json").expect("Failed to open file");
            file.read_to_end(&mut buf).expect("Failed to construct buffer");
            flame::end("fill buffer");
            flame::start("parse into struct");
            let response: RequestResult = serde_json::from_slice(&buf).expect("Failed to deserialize JSON");
            flame::end("parse into struct");
            flame::end("json");
            flame::end("request and json");
            if response.results.len() == 0 {
                // Done
                break;
            }
            connection.execute("BEGIN TRANSACTION").expect("Failed to start transaction");
            flame::start("convert to model");
            for game_result in response.results.into_iter() {
                if let Ok(game_model) = to_model(game_result.data, &game_result.src_abbr) {
                    use schema::games;
                    diesel::replace_into(games::table).values(&game_model).execute(&connection).expect("Error stashing new game");
                } else {
                    println!("Failed to parse, skipping")
                }
            }
            flame::end("convert to model");
            flame::start("execute end transaction");
            connection.execute("END TRANSACTION").expect("Failed to end transaction");
            flame::end("execute end transaction");
            offset = response.next_offset;
            stashed_games += LOG_CHUNK_SIZE;
            if stashed_games % 10000 == 0 {
                println!("{} games parsed in {} secs", stashed_games, start_time.elapsed().as_secs());
            }
        }
        buf.clear();
    }
    flame::end("loop");
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
