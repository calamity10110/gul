use gul_saas::aws::AwsUsageReporter;
use gul_saas::usage::UsageMeter;

#[tokio::main]
async fn main() {
    println!("gul-saas AWS Demo");

    // Simulate usage
    let meter = UsageMeter::new();
    meter.record_hit();
    meter.record_hit();

    println!("Current usage: {}", meter.get_count());

    // Connect to AWS (Mock/Real)
    // In real env, Requires AWS_ACCESS_KEY_ID etc.
    if std::env::var("AWS_ACCESS_KEY_ID").is_ok() {
        let reporter = AwsUsageReporter::new("gul_usage_table").await;
        match reporter.report_usage("tenant-123", &meter).await {
            Ok(_) => println!("Successfully reported to DynamoDB"),
            Err(e) => println!("Failed to report: {}", e),
        }
    } else {
        println!("Skipping AWS report (no credentials)");
    }
}
