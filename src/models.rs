use super::schema::games;

#[derive(Insertable)]
#[table_name = "games"]
pub struct NewGame {
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
