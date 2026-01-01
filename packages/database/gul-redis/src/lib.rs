pub struct RedisManager {
    #[cfg(feature = "real")]
    client: Option<redis::Client>,
    use_mock: bool,
}

impl RedisManager {
    pub fn new(url: &str) -> Result<Self, String> {
        #[cfg(feature = "real")]
        match redis::Client::open(url) {
            Ok(client) => {
                return Ok(Self {
                    client: Some(client),
                    use_mock: false,
                })
            }
            Err(e) => {
                log::warn!("Redis connection failed ({}), using MOCK.", e);
                return Ok(Self {
                    client: None,
                    use_mock: true,
                });
            }
        }

        #[cfg(not(feature = "real"))]
        Ok(Self { use_mock: true })
    }

    pub fn set(&self, key: &str, value: &str) -> Result<(), String> {
        #[cfg(feature = "real")]
        if !self.use_mock {
            if let Some(client) = &self.client {
                let mut con = client.get_connection().map_err(|e| e.to_string())?;
                let _: () =
                    redis::Commands::set(&mut con, key, value).map_err(|e| e.to_string())?;
                return Ok(());
            }
        }
        println!("MOCK SET: {} = {}", key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<String, String> {
        #[cfg(feature = "real")]
        if !self.use_mock {
            if let Some(client) = &self.client {
                let mut con = client.get_connection().map_err(|e| e.to_string())?;
                let val: String = redis::Commands::get(&mut con, key).map_err(|e| e.to_string())?;
                return Ok(val);
            }
        }
        println!("MOCK GET: {}", key);
        Ok("mock_value".to_string())
    }
}
