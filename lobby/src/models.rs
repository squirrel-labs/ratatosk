pub mod db {
    use crate::schema::*;
    #[derive(Queryable, Debug, Identifiable)]
    #[table_name = "game_types"]
    pub struct GameType {
        id: i32,
        name: String,
        redirect_uri: String,
        capacity: Option<i32>,
        color: String,
        icon: String,
    }

    #[derive(Insertable)]
    #[table_name = "game_types"]
    pub struct NewType<'a> {
        pub name: &'a str,
        pub redirect_uri: &'a str,
        pub capacity: Option<i32>,
        pub color: &'a str,
        pub icon: &'a str,
    }

    #[derive(Queryable, Identifiable, Associations)]
    #[belongs_to(GameType)]
    pub struct Group {
        id: i32,
        name: String,
        password: String,
        salt: String,
        game_type_id: i32,
    }

    #[derive(Insertable, Associations)]
    #[table_name = "groups"]
    pub struct NewGroup<'a> {
        name: &'a str,
        password: &'a str,
        salt: &'a str,
        game_type_id: &'a i32,
    }
}
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameType {
    pub name: String,
    pub icon: String,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: u32,
    pub max_users: u32,
    pub user_count: u32,
    pub has_password: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    pub username: String,
    pub name: String,
    pub user_count: u32,
    pub max_users: u32,
    pub has_password: bool,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameOverview {
    pub game_types: Vec<GameType>,
    pub games: Vec<Game>,
}
