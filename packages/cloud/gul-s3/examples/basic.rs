use gul_s3::S3Client;

fn main() {
    let client = S3Client::new("us-east-1", "ACCESS", "SECRET");
    client
        .put_object("my-bucket", "data.txt", b"Hello S3")
        .unwrap();
}
