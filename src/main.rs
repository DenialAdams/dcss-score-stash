#![recursion_limit = "128"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate walkdir;
extern crate notify;

mod schema;
mod models;

use std::io::{BufRead, BufReader};
use walkdir::WalkDir;
use std::fs::File;
use std::str::FromStr;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;

#[derive(Debug)]
struct Morgue {
    name: String,
    version: String,
    score: i64,
    race: Race,
    background: Background,
}

#[derive(Debug, Clone, Copy)]
#[repr(i64)]
enum Race {
    Barachi = 0,
    Centaur,
    DeepDwarf,
    DeepElf,
    Demigod,
    Demonspawn,
    Draconian,
    RedDraconian,
    WhiteDraconian,
    GreenDraconian,
    YellowDraconian,
    GreyDraconian,
    BlackDraconian,
    PurpleDraconian,
    MottledDraconian,
    PaleDraconian,
    Felid,
    Formicid,
    Gargoyle,
    Ghoul,
    Gnoll,
    Halfling,
    HighElf,
    HillOrc,
    Human,
    Kobold,
    Merfolk,
    Minotaur,
    Mummy,
    Naga,
    Ocotopode,
    Ogre,
    Spriggan,
    Tengu,
    Troll,
    Vampire,
    VineStalker,
}

impl FromStr for Race {
    type Err = ();
    fn from_str(s: &str) -> Result<Race, ()> {
        match s {
            "Barachi" => Ok(Race::Barachi),
            "Centaur" => Ok(Race::Centaur),
            "DeepDwarf" => Ok(Race::DeepDwarf),
            "DeepElf" => Ok(Race::DeepElf),
            "Demigod" => Ok(Race::Demigod),
            "Demonspawn" => Ok(Race::Demonspawn),
            "Draconian" => Ok(Race::Draconian),
            "RedDraconian" => Ok(Race::RedDraconian),
            "WhiteDraconian" => Ok(Race::WhiteDraconian),
            "GreenDraconian" => Ok(Race::GreenDraconian),
            "YellowDraconian" => Ok(Race::YellowDraconian),
            "GreyDraconian" => Ok(Race::GreyDraconian),
            "BlackDraconian" => Ok(Race::BlackDraconian),
            "PurpleDraconian" => Ok(Race::PurpleDraconian),
            "MottledDraconian" => Ok(Race::MottledDraconian),
            "PaleDraconian" => Ok(Race::PaleDraconian),
            "Felid" => Ok(Race::Felid),
            "Formicid" => Ok(Race::Formicid),
            "Gargoyle" => Ok(Race::Gargoyle),
            "Ghoul" => Ok(Race::Ghoul),
            "Gnoll" => Ok(Race::Gnoll),
            "Halfling" => Ok(Race::Halfling),
            "HighElf" => Ok(Race::HighElf),
            "HillOrc" => Ok(Race::HillOrc),
            "Human" => Ok(Race::Human),
            "Kobold" => Ok(Race::Kobold),
            "Merfolk" => Ok(Race::Merfolk),
            "Minotaur" => Ok(Race::Minotaur),
            "Mummy" => Ok(Race::Mummy),
            "Naga" => Ok(Race::Naga),
            "Octopode" => Ok(Race::Ocotopode),
            "Ogre" => Ok(Race::Ogre),
            "Spriggan" => Ok(Race::Spriggan),
            "Tengu" => Ok(Race::Tengu),
            "Troll" => Ok(Race::Troll),
            "Vampire" => Ok(Race::Vampire),
            "VineStalker" => Ok(Race::VineStalker),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(i64)]
enum Background {
    Fighter = 0,
    Gladiator,
    Monk,
    Hunter,
    Assassin,
    Berserker,
    AbyssalKnight,
    ChaosKnight,
    Skald,
    Enchanter,
    Transmuter,
    ArcaneMarksman,
    Warper,
    Wizard,
    Conjurer,
    Summoner,
    Necromancer,
    FireElementalist,
    IceElementalist,
    AirElementalist,
    EarthElementalist,
    VenomMage,
    Artificer,
    Wanderer,
}

impl FromStr for Background {
    type Err = ();
    fn from_str(s: &str) -> Result<Background, ()> {
        match s {
            "Fighter" => Ok(Background::Fighter),
            "Gladiator" => Ok(Background::Gladiator),
            "Monk" => Ok(Background::Monk),
            "Hunter" => Ok(Background::Hunter),
            "Assassin" => Ok(Background::Assassin),
            "Berserker" => Ok(Background::Berserker),
            "AbyssalKnight" => Ok(Background::AbyssalKnight),
            "ChaosKnight" => Ok(Background::ChaosKnight),
            "Skald" => Ok(Background::Skald),
            "Enchanter" => Ok(Background::Enchanter),
            "Transmuter" => Ok(Background::Transmuter),
            "ArcaneMarksman" => Ok(Background::ArcaneMarksman),
            "Warper" => Ok(Background::Warper),
            "Wizard" => Ok(Background::Wizard),
            "Conjurer" => Ok(Background::Conjurer),
            "Summoner" => Ok(Background::Summoner),
            "Necromancer" => Ok(Background::Necromancer),
            "FireElementalist" => Ok(Background::FireElementalist),
            "IceElementalist" => Ok(Background::IceElementalist),
            "AirElementalist" => Ok(Background::AirElementalist),
            "EarthElementalist" => Ok(Background::EarthElementalist),
            "VenomMage" => Ok(Background::VenomMage),
            "Artificer" => Ok(Background::Artificer),
            "Wanderer" => Ok(Background::Wanderer),
            _ => Err(()),
        }
    }
}

fn main() {
    dotenv().ok();

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = {
        // TODO, if this fails print an error and do one pass
        Watcher::new(tx, Duration::from_secs(2)).expect("Can't set up file watcher")
    };
    // same here
    watcher.watch("/home/brick/crawl/crawl-ref/source/rcs", notify::RecursiveMode::Recursive).expect("Can't set up file watcher");
    let connection = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    };
    // One pass
    for entry in WalkDir::new("/home/brick/crawl/crawl-ref/source/rcs")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        maybe_parse(entry.path(), &connection);
    }
    // Now check each file as we recv them
    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    notify::DebouncedEvent::Create(path) => {
                        // Try to morgue parse it
                        maybe_parse(&path, &connection);
                    },
                    _ => {
                        continue;
                    }
                }
            }
            Err(e) => {
                panic!(e);
            }
        }
    }
}

#[derive(Debug)]
enum ParseError {
    Io(std::io::Error),
    InvalidMorgue(&'static str),
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

fn maybe_parse(path: &Path, connection: &SqliteConnection) {
    if !path.is_file() {
        return;
    }
    let file_name = if let Some(name) = path.file_name() {
        name
    } else {
        return;
    };
    let file_name = file_name.to_string_lossy();
    if !file_name.starts_with("morgue-") || !file_name.ends_with(".txt") {
        return;
    }
    // Dealing with a bonafide morgue file
    let db_key = file_name.replace("morgue-", "").replace(".txt", "");
    // Continue if it already exists in DB
    {
        let morgue = {
            use schema::morgues::dsl::*;
            morgues
                .find(&db_key)
                .load::<models::DbMorgue>(connection)
                .expect("Error loading morgues")
        };
        if morgue.len() > 0 {
            return;
        }
    }
    let file = BufReader::new(File::open(path).unwrap());
    let morgue = parse(file).expect(&format!("Failed to parse morgue {}", file_name));
    // Stash it in DB
    {
        use schema::morgues;

        let new_morgue = models::NewDbMorgue {
            file_name: &db_key,
            name: &morgue.name,
            version: &morgue.version,
            score: morgue.score as i64,
            race: morgue.race as i64,
            background: morgue.background as i64,
        };

        diesel::insert(&new_morgue)
            .into(morgues::table)
            .execute(connection)
            .expect("Error saving new morgue");
    }
}

// Take in a readable
fn parse<T: BufRead>(handle: T) -> Result<Morgue, ParseError> {
    let mut lines = handle.lines();
    // Parse header
    let version = {
        //  Dungeon Crawl Stone Soup version 0.21-a0-257-gbb444f2580 (webtiles) character file
        let header_line = {
            let foo = lines.next();
            if let Some(line) = foo {
                line?
            } else {
                return Err(ParseError::InvalidMorgue("Morgue was empty"));
            }
        };
        if let Some(version) = header_line
            .split_whitespace()
            .skip_while(|s| *s != "version")
            .skip(1)
            .next()
        {
            String::from(version)
        } else {
            return Err(ParseError::InvalidMorgue(
                "Error while parsing morgue header; can't parse version",
            ));
        }
    };
    // Blank
    lines.next();
    // Parse stats
    let (score, name, race, background) = {
        /*
        5982 brick the Basher (level 11, 0/86 HPs)
             Began as a Formicid Fighter on Sept 6, 2017.
             Was an Elder of Qazlal.
             Slain by a killer bee (4 damage)
             ... on level 8 of the Dungeon.
             The game lasted 00:50:17 (7568 turns).
        */
        // Parse first line
        let (score, name) = {
            let first_line = {
                let foo = lines.next();
                if let Some(line) = foo {
                    line?
                } else {
                    return Err(ParseError::InvalidMorgue("Morgue was empty"));
                }
            };
            let mut line_iter = first_line.split_whitespace();
            let score = if let Some(x) = line_iter.next() {
                if let Ok(val) = x.parse::<i64>() {
                    val
                } else {
                    return Err(ParseError::InvalidMorgue(
                        "Error while parsing morgue stats; can't parse score",
                    ));
                }
            } else {
                // TODO parse error
                return Err(ParseError::InvalidMorgue(
                    "Error while parsing morgue stats; can't parse score",
                ));
            };
            let name = if let Some(x) = line_iter.next() {
                String::from(x)
            } else {
                return Err(ParseError::InvalidMorgue(
                    "Error while parsing morgue stats; can't parse name",
                ));
            };
            (score, name)
        };
        // Parse second line
        let (race, background) = {
            let line = {
                let foo = lines.next();
                if let Some(line) = foo {
                    line?
                } else {
                    return Err(ParseError::InvalidMorgue(
                        "Morgue stats is missing information",
                    ));
                }
            };
            let mut line_iter = line.split_whitespace().skip(3);
            // Race and Background can total 2 - 4 words, so we play this annyoing try parse game
            let race_word = if let Some(x) = line_iter.next() {
                x
            } else {
                return Err(ParseError::InvalidMorgue(
                    "Error while parsing morgue stats; Can't parse race",
                ));
            };
            let race = if let Ok(x) = race_word.parse::<Race>() {
                x
            } else {
                // Race must be two words
                let race_word_2 = if let Some(x) = line_iter.next() {
                    x
                } else {
                    return Err(ParseError::InvalidMorgue(
                        "Error while parsing morgue stats; Can't parse race",
                    ));
                };
                let race_string = format!("{}{}", race_word, race_word_2);
                if let Ok(x) = race_string.parse::<Race>() {
                    x
                } else {
                    return Err(ParseError::InvalidMorgue(
                        "Error while parsing morgue stats; Can't parse race",
                    ));
                }
            };
            let bg_word = if let Some(x) = line_iter.next() {
                x
            } else {
                return Err(ParseError::InvalidMorgue(
                    "Error while parsing morgue stats; Can't parse background",
                ));
            };
            let bg = if let Ok(x) = bg_word.parse::<Background>() {
                x
            } else {
                // Race must be two words
                let bg_word_2 = if let Some(x) = line_iter.next() {
                    x
                } else {
                    return Err(ParseError::InvalidMorgue(
                        "Error while parsing morgue stats; Can't parse background",
                    ));
                };
                let bg_string = format!("{}{}", bg_word, bg_word_2);
                if let Ok(x) = bg_string.parse::<Background>() {
                    x
                } else {
                    return Err(ParseError::InvalidMorgue(
                        "Error while parsing morgue stats; Can't parse background",
                    ));
                }
            };
            (race, bg)
        };
        (score, name, race, background)
    };
    Ok(Morgue {
        name: name,
        version: version,
        score: score,
        background: background,
        race: race,
    })
}
