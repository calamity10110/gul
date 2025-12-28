# Data Engineering with GUL - Complete Guide

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

Build production-grade data pipelines for SaaS platforms with GUL.

---

## Table of Contents

1. [Overview](#overview)
2. [ETL Pipelines](#etl-pipelines)
3. [Stream Processing](#stream-processing)
4. [Multi-Tenant Analytics](#multi-tenant-analytics)
5. [Cloud Warehouses](#cloud-warehouses)
6. [Workflow Orchestration](#workflow-orchestration)
7. [Best Practices](#best-practices)

---

## Overview

GUL provides comprehensive data engineering capabilities for modern SaaS platforms:

- **ETL/ELT Pipelines** - Extract, transform, load data at scale
- **Stream Processing** - Real-time event processing with Kafka
- **Batch Processing** - Big data with Spark and Databricks
- **Cloud Warehouses** - Snowflake, BigQuery, Redshift integration
- **Orchestration** - Airflow, dbt, custom workflows

---

## ETL Pipelines

### Basic ETL Pattern

```gul
@imp gul.etl
@imp gul.postgres
@imp python{pandas}

struct DataPipeline:
    source_db: @str
    target_db: @str

    async fn @dict extract(self, query):
        """Extract data from source"""
        let conn = postgres.connect(self.source_db)
        let data = await conn.query(query)
        return data

    fn @dict transform(self, data):
        """Transform data with Python/Pandas"""
        @python {
            df = pd.DataFrame(data)

            # Clean data
            df = df.dropna()
            df = df.drop_duplicates()

            # Transform
            df['revenue'] = df['price'] * df['quantity']
            df['date'] = pd.to_datetime(df['timestamp'])

            # Aggregate
            daily_revenue = df.groupby('date')['revenue'].sum()

            return daily_revenue.to_dict()
        }
        return python.result

    async fn @bool load(self, data):
        """Load data to target"""
        let conn = postgres.connect(self.target_db)

        for date, revenue in data.items():
            await conn.execute(
                "INSERT INTO daily_revenue (date, amount) VALUES ($1, $2)",
                @list[date, revenue]
            )

        return @bool(true)

# Run pipeline
async main():
    let pipeline = DataPipeline{
        source_db: "postgresql://prod/sales",
        target_db: "postgresql://warehouse/analytics"
    }

    # Extract
    let raw_data = await pipeline.extract("SELECT * FROM transactions")

    # Transform
    let clean_data = pipeline.transform(raw_data)

    # Load
    await pipeline.load(clean_data)

    print("Pipeline complete!")

mn:
    await main()
```

### Multi-Source ETL

```gul
@imp gul{etl, postgres, mongodb, redis}
@imp python{pandas, numpy}

struct MultiSourcePipeline:

    async fn @dict extract_all(self):
        """Extract from multiple sources in parallel"""

        # PostgreSQL data
        async fn @dict get_postgres():
            let conn = postgres.connect("postgresql://localhost/orders")
            return await conn.query("SELECT * FROM orders")

        # MongoDB data
        async fn @dict get_mongo():
            let client = mongodb.connect("mongodb://localhost/users")
            return await client.find(@dict{})

        # Redis cache
        async fn @dict get_redis():
            let client = redis.connect("redis://localhost")
            let keys = await client.keys("session:*")
            let sessions = @dict{}

            for key in keys:
                sessions[key] = await client.get(key)

            return sessions

        # Run in parallel
        let results = await async.gather(@list[
            get_postgres(),
            get_mongo(),
            get_redis()
        ])

        return @dict{
            postgres: results[0],
            mongodb: results[1],
            redis: results[2]
        }

    fn @dict transform_unified(self, sources):
        """Unify data from multiple sources"""
        @python {
            # Convert to DataFrames
            orders_df = pd.DataFrame(sources['postgres'])
            users_df = pd.DataFrame(sources['mongodb'])
            sessions_data = sources['redis']

            # Join data
            unified = orders_df.merge(
                users_df,
                left_on='user_id',
                right_on='_id',
                how='left'
            )

            # Add session data
            unified['session_count'] = unified['user_id'].map(
                lambda uid: len([k for k in sessions_data if str(uid) in k])
            )

            # Compute metrics
            unified['ltv'] = unified.groupby('user_id')['total'].transform('sum')

            return unified.to_dict('records')
        }
        return python.result

mn:
    let pipeline = MultiSourcePipeline{}
    let sources = await pipeline.extract_all()
    let unified = pipeline.transform_unified(sources)
    print("Processed", len(unified), "records")
```

---

## Stream Processing

### Kafka Consumer

```gul
@imp gul.kafka
@imp gul.postgres

struct EventProcessor:
    topic: @str
    consumer_group: @str
    db_conn: @str

    async fn process_events(self):
        """Process streaming events from Kafka"""

        # Create Kafka consumer
        let consumer = kafka.Consumer{
            bootstrap_servers: @list["localhost:9092"],
            group_id: self.consumer_group,
            auto_offset_reset: "earliest"
        }

        consumer.subscribe(@list[self.topic])

        # Database connection
        let db = postgres.connect(self.db_conn)

        # Process stream
        while true:
            let messages = await consumer.poll(timeout=1.0)

            for message in messages:
                let event = json.parse(message.value)

                # Real-time processing
                match event["type"]:
                    "user_signup" => self.handle_signup(db, event)
                    "purchase" => self.handle_purchase(db, event)
                    "page_view" => self.handle_pageview(db, event)
                    _ => print("Unknown event:", event["type"])

    fn handle_purchase(self, db, event):
        """Handle purchase event"""
        # Update user LTV
        db.execute(
            "UPDATE users SET ltv = ltv + $1 WHERE id = $2",
            @list[event["amount"], event["user_id"]]
        )

        # Record in analytics
        db.execute(
            "INSERT INTO purchases (user_id, amount, timestamp) VALUES ($1, $2, $3)",
            @list[event["user_id"], event["amount"], time.now()]
        )

# Run processor
mn:
    let processor = EventProcessor{
        topic: "user-events",
        consumer_group: "analytics-processor",
        db_conn: "postgresql://localhost/analytics"
    }

    await processor.process_events()
```

### Stream Aggregation

```gul
@imp gul{kafka, redis}

struct StreamAggregator:
    """Real-time windowed aggregations"""

    window_size: @int  # seconds

    async fn aggregate_realtime(self, topic):
        """Compute real-time metrics with sliding windows"""

        let consumer = kafka.Consumer{
            bootstrap_servers: @list["localhost:9092"],
            group_id: "aggregator"
        }

        let cache = redis.connect("redis://localhost")
        consumer.subscribe(@list[topic])

        while true:
            let messages = await consumer.poll(timeout=0.1)

            for msg in messages:
                let event = json.parse(msg.value)
                let window_key = self.get_window_key(event["timestamp"])

                # Increment counters in Redis
                await cache.hincrby(window_key, "count", 1)
                await cache.hincrbyfloat(window_key, "sum", event["value"])

                # Set expiry
                await cache.expire(window_key, self.window_size * 2)

                # Compute running average
                let count = await cache.hget(window_key, "count")
                let sum = await cache.hget(window_key, "sum")
                let avg = sum / count

                # Publish aggregated result
                await cache.publish("metrics", json.stringify(@dict{
                    window: window_key,
                    count: count,
                    average: avg,
                    timestamp: time.now()
                }))

    fn @str get_window_key(self, timestamp):
        """Get window key for timestamp"""
        let window_start = (timestamp / self.window_size) * self.window_size
        return "window:" + str(window_start)

mn:
    let aggregator = StreamAggregator{window_size: 60}  # 1-minute windows
    await aggregator.aggregate_realtime("metrics")
```

---

## Multi-Tenant Analytics

### Tenant Isolation

```gul
@imp gul{postgres, snowflake}

struct TenantAnalytics:
    """Multi-tenant analytics with row-level security"""

    fn @str get_tenant_schema(self, tenant_id):
        """Get schema name for tenant"""
        return "tenant_" + str(tenant_id)

    async fn @dict query_tenant_data(self, tenant_id, query):
        """Query data with tenant isolation"""

        let conn = snowflake.connect("snowflake://account/db")
        let schema = self.get_tenant_schema(tenant_id)

        # Set session context
        await conn.execute("USE SCHEMA " + schema)

        # Execute query (only sees tenant data)
        let result = await conn.query(query)

        return result

    async fn compute_tenant_metrics(self, tenant_id):
        """Compute metrics for single tenant"""

        let metrics = @dict{}

        # Active users
        metrics["active_users"] = await self.query_tenant_data(
            tenant_id,
            "SELECT COUNT(DISTINCT user_id) FROM events WHERE date >= CURRENT_DATE - 30"
        )

        # Revenue
        metrics["mrr"] = await self.query_tenant_data(
            tenant_id,
            "SELECT SUM(amount) FROM subscriptions WHERE status = 'active'"
        )

        # Engagement
        metrics["dau"] = await self.query_tenant_data(
            tenant_id,
            "SELECT COUNT(DISTINCT user_id) FROM events WHERE date = CURRENT_DATE"
        )

        return metrics

# Usage
mn:
    let analytics = TenantAnalytics{}

    # Get metrics for tenant
    let metrics = await analytics.compute_tenant_metrics(tenant_id=123)

    print("Tenant 123 Metrics:", metrics)
```

### Cross-Tenant Aggregation

```gul
@imp gul{bigquery}
@imp python{pandas}

async fn analyze_all_tenants():
    """Aggregate metrics across all tenants"""

    let client = bigquery.connect("project-id")

    # Query across all tenant tables
    let query = """
        SELECT
            tenant_id,
            COUNT(DISTINCT user_id) as users,
            SUM(revenue) as revenue,
            AVG(engagement_score) as avg_engagement
        FROM `analytics.tenant_*`
        WHERE date >= DATE_SUB(CURRENT_DATE(), INTERVAL 30 DAY)
        GROUP BY tenant_id
        ORDER BY revenue DESC
    """

    let results = await client.query(query)

    @python {
        df = pd.DataFrame(results)

        # Compute percentiles
        df['revenue_percentile'] = df['revenue'].rank(pct=True)
        df['users_percentile'] = df['users'].rank(pct=True)

        # Segment tenants
        df['segment'] = 'Low'
        df.loc[df['revenue_percentile'] > 0.5, 'segment'] = 'Medium'
        df.loc[df['revenue_percentile'] > 0.8, 'segment'] = 'High'
        df.loc[df['revenue_percentile'] > 0.95, 'segment'] = 'Enterprise'

        # Compute cohort metrics
        metrics = {
            'total_tenants': len(df),
            'segments': df.groupby('segment').agg({
                'tenant_id': 'count',
                'revenue': 'sum',
                'users': 'sum'
            }).to_dict()
        }

        return metrics
    }

    return python.metrics

mn:
    let all_tenant_metrics = await analyze_all_tenants()
    print("Cross-tenant metrics:", all_tenant_metrics)
```

---

## Cloud Warehouses

### Snowflake Integration

```gul
@imp gul.snowflake

struct SnowflakeWarehouse:
    account: @str
    database: @str
    warehouse: @str

    async fn create_fact_table(self):
        """Create optimized fact table"""

        let conn = snowflake.connect(
            account=self.account,
            database=self.database,
            warehouse=self.warehouse
        )

        await conn.execute("""
            CREATE TABLE IF NOT EXISTS fact_events (
                event_id STRING,
                tenant_id NUMBER,
                user_id NUMBER,
                event_type STRING,
                properties VARIANT,
                timestamp TIMESTAMP_NTZ,
                date DATE
            )
            CLUSTER BY (tenant_id, date)
            PARTITION BY (date)
        """)

    async fn load_from_s3(self, s3_path):
        """Load data from S3 using COPY command"""

        let conn = snowflake.connect(account=self.account)

        await conn.execute("""
            COPY INTO fact_events
            FROM '{}'
            FILE_FORMAT = (TYPE = 'PARQUET')
            MATCH_BY_COLUMN_NAME = CASE_INSENSITIVE
        """.format(s3_path))

    async fn @dict query_with_caching(self, query):
        """Execute query with result caching"""

        let conn = snowflake.connect(account=self.account)

        # Enable result caching
        await conn.execute("ALTER SESSION SET USE_CACHED_RESULT = TRUE")

        # Execute query
        let result = await conn.query(query)

        return result
```

### BigQuery Streaming

```gul
@imp gul.bigquery

struct BigQueryStreaming:
    project: @str
    dataset: @str
    table: @str

    async fn stream_insert(self, rows):
        """Stream data to BigQuery in real-time"""

        let client = bigquery.connect(self.project)
        let table_ref = "{}.{}.{}".format(self.project, self.dataset, self.table)

        # Prepare rows with insert IDs for deduplication
        let rows_to_insert = @list[]

        for row in rows:
            rows_to_insert.append(@dict{
                insert_id: row["id"],
                json: row
            })

        # Stream insert
        let errors = await client.insert_rows(table_ref, rows_to_insert)

        if errors:
            print("Errors:", errors)
            return @bool(false)

        return @bool(true)

    async fn query_realtime(self, sql):
        """Query with real-time data"""

        let client = bigquery.connect(self.project)

        # Query config for real-time
        let job_config = @dict{
            use_legacy_sql: false,
            use_query_cache: false  # Get freshest data
        }

        let result = await client.query(sql, job_config)
        return result
```

---

## Workflow Orchestration

### Airflow DAG

```gul
@imp gul.airflow

fn create_etl_dag():
    """Define Airflow DAG for ETL pipeline"""

    let dag = airflow.DAG{
        dag_id: "saas_etl_pipeline",
        schedule_interval: "0 2 * * *",  # Daily at 2 AM
        default_args: @dict{
            owner: "data-team",
            retries: 3,
            retry_delay: 300  # 5 minutes
        }
    }

    # Task 1: Extract from sources
    let extract_task = airflow.Task{
        task_id: "extract_data",
        python_callable: extract_all_sources,
        provide_context: true
    }

    # Task 2: Transform data
    let transform_task = airflow.Task{
        task_id: "transform_data",
        python_callable: transform_and_clean,
        provide_context: true
    }

    # Task 3: Load to warehouse
    let load_task = airflow.Task{
        task_id: "load_to_warehouse",
        python_callable: load_to_snowflake,
        provide_context: true
    }

    # Task 4: Run dbt models
    let dbt_task = airflow.Task{
        task_id: "run_dbt_models",
        bash_command: "cd /dbt && dbt run --profiles-dir ."
    }

    # Task 5: Data quality checks
    let quality_task = airflow.Task{
        task_id: "data_quality_checks",
        python_callable: run_quality_checks
    }

    # Define dependencies
    extract_task >> transform_task >> load_task >> dbt_task >> quality_task

    return dag
```

### dbt Models

```gul
@imp gul.dbt

struct DbtProject:
    project_dir: @str

    fn create_model(self, name, sql):
        """Create dbt model"""

        let model_path = self.project_dir + "/models/" + name + ".sql"

        fs.write_file(model_path, sql)

    async fn @dict run_models(self, models):
        """Run specific dbt models"""

        let result = await sys.execute(
            "dbt run --models " + models.join(" "),
            cwd=self.project_dir
        )

        return @dict{
            success: result.exit_code == 0,
            output: result.output
        }

    async fn @dict test_models(self):
        """Run dbt tests"""

        let result = await sys.execute("dbt test", cwd=self.project_dir)

        return @dict{
            tests_passed: result.exit_code == 0,
            output: result.output
        }

# Usage
mn:
    let project = DbtProject{project_dir: "/data/dbt"}

    # Create model
    project.create_model("daily_revenue", """
        SELECT
            DATE(timestamp) as date,
            SUM(amount) as revenue,
            COUNT(DISTINCT user_id) as unique_users
        FROM {{ ref('raw_transactions') }}
        GROUP BY 1
    """)

    # Run and test
    await project.run_models(@list["daily_revenue"])
    let test_results = await project.test_models()

    print("Tests:", test_results)
```

---

## Best Practices

### 1. Idempotent Pipelines

```gul
async fn idempotent_load(data, table, partition_key):
    """Load data idempotently using upserts"""

    let conn = postgres.connect("postgresql://warehouse/analytics")

    # Use UPSERT to handle duplicates
    for row in data:
        await conn.execute("""
            INSERT INTO {} (id, data, partition, updated_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (id, partition)
            DO UPDATE SET
                data = EXCLUDED.data,
                updated_at = NOW()
        """.format(table), @list[
            row["id"],
            json.stringify(row),
            row[partition_key]
        ])
```

### 2. Error Handling

```gul
struct RobustPipeline:
    max_retries: @int

    async fn @dict run_with_retry(self, fn, *args):
        """Run function with exponential backoff"""

        var retries = 0

        while retries < self.max_retries:
            try:
                return await fn(*args)
            catch error:
                retries = retries + 1

                if retries >= self.max_retries:
                    # Send alert
                    self.send_alert(error)
                    raise error

                # Exponential backoff
                let wait_time = math.pow(2, retries)
                await time.sleep(wait_time)
```

### 3. Monitoring

```gul
struct PipelineMonitor:

    fn track_metrics(self, pipeline_name, metrics):
        """Track pipeline metrics"""

        # Send to monitoring service
        let influx = influxdb.connect("influxdb://localhost")

        influx.write(@dict{
            measurement: "pipeline_metrics",
            tags: @dict{pipeline: pipeline_name},
            fields: @dict{
                records_processed: metrics["count"],
                duration_seconds: metrics["duration"],
                error_count: metrics["errors"]
            },
            timestamp: time.now()
        })
```

---

## See Also

- [Microservices Guide](microservices-guide.md)
- [Package Catalog](../reference/package-catalog.md)
- [Standard Library](../api/standard-library.md)

---

**Last Updated**: 2025-12-28  
**Version**: 0.13.0  
**Status**: Production Ready
