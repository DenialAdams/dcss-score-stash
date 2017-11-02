use super::schema::games;

#[derive(Insertable)]
#[table_name = "games"]
pub struct NewGame {
    pub gid: String,
    pub xl: i64,
    pub score: i64
}
