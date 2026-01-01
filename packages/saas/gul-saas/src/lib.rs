use serde::{Deserialize, Serialize};

pub mod tenants {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tenant {
        pub id: String,
        pub name: String,
        pub plan: SubscriptionPlan,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum SubscriptionPlan {
        Free,
        Pro,
        Enterprise,
    }

    impl Tenant {
        pub fn new(id: &str, name: &str, plan: SubscriptionPlan) -> Self {
            Self {
                id: id.to_string(),
                name: name.to_string(),
                plan,
            }
        }

        pub fn can_access_feature(&self, feature: &str) -> bool {
            match self.plan {
                SubscriptionPlan::Free => feature == "basic_stats",
                SubscriptionPlan::Pro => feature != "audit_logs",
                SubscriptionPlan::Enterprise => true,
            }
        }
    }
}

pub mod billing {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Invoice {
        pub id: String,
        pub tenant_id: String,
        pub amount: f64,
        pub currency: String,
        pub status: InvoiceStatus,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum InvoiceStatus {
        Pending,
        Paid,
        Overdue,
    }
}

pub mod analytics {
    pub struct Event {
        pub name: String,
        pub properties: std::collections::HashMap<String, String>,
    }
}

pub mod usage {
    use std::sync::atomic::{AtomicU64, Ordering};

    pub struct UsageMeter {
        counter: AtomicU64,
    }

    impl UsageMeter {
        pub fn new() -> Self {
            Self {
                counter: AtomicU64::new(0),
            }
        }

        pub fn record_hit(&self) {
            self.counter.fetch_add(1, Ordering::Relaxed);
        }

        pub fn get_count(&self) -> u64 {
            self.counter.load(Ordering::Relaxed)
        }
    }
}

pub mod aws {
    use crate::usage::UsageMeter;
    use aws_config::meta::region::RegionProviderChain;
    use aws_sdk_dynamodb::types::AttributeValue;
    use aws_sdk_dynamodb::Client;

    pub struct AwsUsageReporter {
        client: Client,
        table_name: String,
    }

    impl AwsUsageReporter {
        pub async fn new(table_name: &str) -> Self {
            let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
            let config = aws_config::from_env().region(region_provider).load().await;
            let client = Client::new(&config);

            Self {
                client,
                table_name: table_name.to_string(),
            }
        }

        pub async fn report_usage(
            &self,
            tenant_id: &str,
            meter: &UsageMeter,
        ) -> Result<(), String> {
            let count = meter.get_count();

            self.client
                .put_item()
                .table_name(&self.table_name)
                .item("tenant_id", AttributeValue::S(tenant_id.to_string()))
                .item("usage_count", AttributeValue::N(count.to_string()))
                .item(
                    "timestamp",
                    AttributeValue::S(chrono::Utc::now().to_rfc3339()),
                )
                .send()
                .await
                .map_err(|e| e.to_string())?;

            Ok(())
        }
    }
}
