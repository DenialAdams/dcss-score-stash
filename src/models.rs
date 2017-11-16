use super::schema::games;
use super::schema::species;

#[derive(Insertable)]
#[table_name = "games"]
#[belongs_to(Species)]
pub struct NewGame<'a> {
    pub gid: &'a str,
    pub species_id: i32,
    pub xl: i64,
    pub tmsg: &'a str,
    pub turn: i64,
    pub score: i64,
    pub start: &'a str,
    pub end: &'a str,
    pub potions_used: i64,
    pub scrolls_used: i64,
    pub dam: i64,
    pub tdam: i64,
    pub sdam: i64,
    pub dur: i64,
    pub runes: i64,
}

#[derive(Queryable, Associations)]
pub struct Game {
    pub gid: String,
    pub xl: i64,
    pub tmsg: String,
    pub turn: i64,
    pub score: i64,
    pub start: String,
    pub end: String,
    pub potions_used: i64,
    pub scrolls_used: i64,
    pub dam: i64,
    pub tdam: i64,
    pub sdam: i64,
    pub dur: i64,
    pub runes: i64,
}

#[derive(Insertable)]
#[table_name = "species"]
pub struct NewSpecies<'a> {
    pub short: &'a str,
    pub name: &'a str,
    pub playable: i64,
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name = "species"]
pub struct Species {
    pub id: i32,
    pub short: String,
    pub name: String,
    pub playable: i64,
}
