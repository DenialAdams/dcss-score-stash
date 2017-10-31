use super::schema::morgues;

#[derive(Insertable)]
#[table_name = "morgues"]
pub struct NewDbMorgue<'a> {
    pub file_name: &'a str,
    pub name: &'a str,
    pub version: &'a str,
    pub score: i64,
    pub race: i64,
    pub background: i64,
}

#[derive(Queryable)]
pub struct DbMorgue {
    pub file_name: String,
    pub name: String,
    pub version: String,
    pub score: i64,
    pub race: i64,
    pub background: i64,
}
