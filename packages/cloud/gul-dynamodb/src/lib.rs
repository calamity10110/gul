use serde_json::Value;

pub struct DynamoClient {
    region: String,
}

impl DynamoClient {
    pub fn new(region: &str) -> Self {
        Self {
            region: region.to_string(),
        }
    }

    pub fn put_item(&self, table: &str, item: &Value) -> Result<(), String> {
        println!("MOCK: Put item into table '{}': {:?}", table, item);
        Ok(())
    }

    pub fn get_item(&self, table: &str, key: &str) -> Result<Value, String> {
        println!("MOCK: Get item from table '{}' with key '{}'", table, key);
        Ok(serde_json::json!({ "id": key, "value": "mock_data" }))
    }
}
