use gul_dynamodb::DynamoClient;
use serde_json::json;

fn main() {
    let client = DynamoClient::new("us-west-2");
    let item = json!({ "id": "123", "name": "Test User" });
    client.put_item("Users", &item).unwrap();
}
