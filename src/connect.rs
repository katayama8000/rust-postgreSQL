use anyhow::Result;
use postgres::{Client, Config, NoTls};

use crate::params::ConnectionParams;

pub struct PostgresClient;
impl PostgresClient {
    pub fn connect(params: &ConnectionParams) -> Result<Client> {
        let client = Client::connect(params.connect_string().as_str(), NoTls)?;
        Ok(client)
    }

    pub fn config_connect(params: &ConnectionParams) -> Result<Client> {
        let client = Config::new()
            .host(&params.host)
            .port(params.port)
            .dbname(&params.dbname)
            .user(&params.user)
            .password(&params.password)
            .connect(NoTls)?;
        Ok(client)
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::ConnectionParams;

    #[test]
    fn connect_ok() {
        let params = ConnectionParams::new(
            "localhost".to_string(),
            5432,
            "postgres".to_string(),
            "user".to_string(),
            "password".to_string(),
        );
        match PostgresClient::connect(&params) {
            Ok(client) => {
                println!("connect ok");
                let _ = client.close();
            }
            Err(_) => {
                panic!("connect error");
            }
        }
        match PostgresClient::config_connect(&params) {
            Ok(client) => {
                println!("config_connect ok");
                let _ = client.close();
            }
            Err(_) => {
                panic!("config_connect error");
            }
        }
    }
}
