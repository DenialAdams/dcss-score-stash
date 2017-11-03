use super::schema::games;

#[derive(Insertable)]
#[table_name = "games"]
pub struct NewGame<'a> {
    pub gid: String,
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
