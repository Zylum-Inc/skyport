use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostgresConf{
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String
}