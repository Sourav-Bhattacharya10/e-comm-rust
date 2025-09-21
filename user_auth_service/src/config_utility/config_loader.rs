use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigLoader {
    pub server_host: String,
    pub server_port: u16,
    pub server_user: String,
    pub server_password: String,
    pub server_db_name: String,
    pub server_db_schema: String,
}
