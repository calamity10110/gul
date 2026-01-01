use gul_postgres::Connection;

#[tokio::main]
async fn main() {
    let mut conn = Connection::new("mydb", "myuser");

    // Default is MOCK mode, so this won't actually fail network
    conn.connect().await.unwrap();
    println!("Connected (Mock): {}", true);
}
