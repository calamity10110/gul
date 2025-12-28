# Polyglot Microservices with GUL

**Version**: 0.13.0 | **Syntax**:v3.2 | **Updated**: 2025-12-28

Build production-grade polyglot microservices architectures with GUL.

---

## Table of Contents

1. [Overview](#overview)
2. [gRPC Services](#grpc-services)
3. [Service Mesh](#service-mesh)
4. [Service Discovery](#service-discovery)
5. [API Gateway](#api-gateway)
6. [Distributed Transactions](#distributed-transactions)
7. [Best Practices](#best-practices)

---

## Overview

GUL enables building polyglot microservices with:

- **gRPC** - High-performance RPC
- **Service Mesh** - Istio, Linkerd, Envoy
- **Message Queues** - NATS, Kafka
- **Service Discovery** - Consul, Dapr
- **API Gateway** - Routing, authentication, rate limiting

---

## gRPC Services

### Define Service (Protocol Buffers)

```protobuf
// user.proto
syntax = "proto3";

package user;

service UserService {
    rpc GetUser(GetUserRequest) returns (User);
    rpc CreateUser(CreateUserRequest) returns (User);
    rpc ListUsers(ListUsersRequest) returns (stream User);
}

message User {
    string id = 1;
    string name = 2;
    string email = 3;
    int64 created_at = 4;
}

message GetUserRequest {
    string id = 1;
}

message CreateUserRequest {
    string name = 1;
    string email = 2;
}

message ListUsersRequest {
    int32 page_size = 1;
    string page_token = 2;
}
```

### Implement Service in GUL

```gul
@imp gul{grpc, proto, postgres}

struct UserService:
    db_conn: @str

    async fn @dict get_user(self, request):
        """Get user by ID"""

        let conn = postgres.connect(self.db_conn)

        let user = await conn.query_one(
            "SELECT * FROM users WHERE id = $1",
            @list[request.id]
        )

        if not user:
            raise grpc.NotFoundError("User not found")

        return @dict{
            id: user["id"],
            name: user["name"],
            email: user["email"],
            created_at: user["created_at"]
        }

    async fn @dict create_user(self, request):
        """Create new user"""

        let conn = postgres.connect(self.db_conn)

        let user_id = crypto.uuid()

        await conn.execute(
            "INSERT INTO users (id, name, email, created_at) VALUES ($1, $2, $3, NOW())",
            @list[user_id, request.name, request.email]
        )

        return await self.get_user(@dict{id: user_id})

    async fn stream_users(self, request):
        """Stream users with pagination"""

        let conn = postgres.connect(self.db_conn)

        var offset = 0
        let page_size = request.page_size or 100

        while true:
            let users = await conn.query(
                "SELECT * FROM users LIMIT $1 OFFSET $2",
                @list[page_size, offset]
            )

            if len(users) == 0:
                break

            for user in users:
                yield @dict{
                    id: user["id"],
                    name: user["name"],
                    email: user["email"],
                    created_at: user["created_at"]
                }

            offset = offset + page_size

# Start gRPC server
mn:
    let service = UserService{
        db_conn: "postgresql://localhost/users"
    }

    let server = grpc.Server{
        port: 50051,
        services: @list[
            grpc.register_service("user.UserService", service)
        ]
    }

    print("gRPC server listening on :50051")
    await server.serve()
```

### gRPC Client

```gul
@imp gul.grpc

async fn call_user_service():
    """Call UserService from another microservice"""

    # Create gRPC channel
    let channel = grpc.Channel("user-service:50051")
    let client = grpc.Client(channel, "user.UserService")

    # Create user
    let new_user = await client.CreateUser(@dict{
        name: "Alice",
        email: "alice@example.com"
    })

    print("Created user:", new_user.id)

    # Get user
    let user = await client.GetUser(@dict{id: new_user.id})
    print("User:", user.name)

    # Stream users
    async for user in client.ListUsers(@dict{page_size: 10}):
        print("User:", user.name)

mn:
    await call_user_service()
```

---

## Service Mesh

### Istio Configuration

```gul
@imp gul.istio

fn configure_istio():
    """Configure Istio for microservices"""

    # Virtual Service for routing
    let virtual_service = istio.VirtualService{
        name: "user-service",
        gateways: @list["public-gateway"],
        http: @list[
            @dict{
                match: @list[@dict{uri: @dict{prefix: "/api/v1/users"}}],
                route: @list[
                    @dict{
                        destination: @dict{
                            host: "user-service",
                            subset: "v1"
                        },
                        weight: 90
                    },
                    @dict{
                        destination: @dict{
                            host: "user-service",
                            subset: "v2"
                        },
                        weight: 10  # Canary deployment
                    }
                ]
            }
        ]
    }

    # Destination Rule for load balancing
    let destination_rule = istio.DestinationRule{
        name: "user-service",
        host: "user-service",
        trafficPolicy: @dict{
            loadBalancer: @dict{
                simple: "LEAST_REQUEST"
            },
            connectionPool: @dict{
                tcp: @dict{maxConnections: 100},
                http: @dict{
                    http1MaxPendingRequests: 50,
                    http2MaxRequests: 100
                }
            }
        },
        subsets: @list[
            @dict{name: "v1", labels: @dict{version: "v1"}},
            @dict{name: "v2", labels: @dict{version: "v2"}}
        ]
    }

    # Apply configurations
    istio.apply(virtual_service)
    istio.apply(destination_rule)
```

### Circuit Breaker

```gul
@imp gul.envoy

struct CircuitBreaker:
    service_name: @str

    fn configure(self):
        """Configure circuit breaker with Envoy"""

        let config = envoy.CircuitBreakerConfig{
            thresholds: @dict{
                max_connections: 1000,
                max_pending_requests: 100,
                max_requests: 1000,
                max_retries: 3
            },
            outlier_detection: @dict{
                consecutive_errors: 5,
                interval: 30,  # seconds
                base_ejection_time: 30,
                max_ejection_percent: 50
            }
        }

        envoy.apply_circuit_breaker(self.service_name, config)
```

---

## Service Discovery

### Consul Integration

```gul
@imp gul.consul

struct ServiceRegistry:
    consul_addr: @str

    async fn register_service(self, name, port, health_check):
        """Register service with Consul"""

        let client = consul.connect(self.consul_addr)

        let service = @dict{
            name: name,
            port: port,
            address: sys.get_hostname(),
            check: @dict{
                http: health_check,
                interval: "10s",
                timeout: "2s",
                deregister_critical_service_after: "1m"
            },
            tags: @list["v1", "production"]
        }

        await client.agent.service_register(service)
        print("Registered service:", name)

    async fn discover_service(self, name):
        """Discover service instances"""

        let client = consul.connect(self.consul_addr)

        let services = await client.health.service(name, passing=true)

        let instances = @list[]
        for service in services:
            instances.append(@dict{
                address: service.Service.Address,
                port: service.Service.Port
            })

        return instances

    async fn get_kv(self, key):
        """Get configuration from KV store"""

        let client = consul.connect(self.consul_addr)
        let value = await client.kv.get(key)

        return value

# Usage
mn:
    let registry = ServiceRegistry{
        consul_addr: "http://consul:8500"
    }

    # Register this service
    await registry.register_service(
        name="user-service",
        port=8080,
        health_check="http://localhost:8080/health"
    )

    # Discover other services
    let auth_instances = await registry.discover_service("auth-service")
    print("Auth service instances:", auth_instances)
```

### Dapr Sidecar

```gul
@imp gul.dapr

struct DaprService:
    app_id: @str
    dapr_port: @int

    async fn call_service(self, service_id, method, data):
        """Call another service via Dapr"""

        let client = dapr.Client(self.dapr_port)

        let response = await client.invoke_method(
            app_id=service_id,
            method=method,
            data=data
        )

        return response

    async fn publish_event(self, topic, data):
        """Publish event to pub/sub"""

        let client = dapr.Client(self.dapr_port)

        await client.publish_event(
            pubsub_name="pubsub",
            topic=topic,
            data=data
        )

    async fn save_state(self, key, value):
        """Save state to state store"""

        let client = dapr.Client(self.dapr_port)

        await client.save_state(
            store_name="statestore",
            key=key,
            value=value
        )

    async fn get_state(self, key):
        """Get state from state store"""

        let client = dapr.Client(self.dapr_port)

        return await client.get_state(
            store_name="statestore",
            key=key
        )

# Usage
mn:
    let service = DaprService{
        app_id: "user-service",
        dapr_port: 3500
    }

    # Call auth service
    let auth_result = await service.call_service(
        service_id="auth-service",
        method="validate-token",
        data=@dict{token: "abc123"}
    )

    # Publish event
    await service.publish_event(
        topic="user-created",
        data=@dict{user_id: "123", email: "user@example.com"}
    )

    # Save state
    await service.save_state("user:123:session", @dict{
        logged_in: true,
        expires_at: time.now() + 3600
    })
```

---

## API Gateway

### Custom API Gateway

```gul
@imp gul{http, consul, redis}

struct APIGateway:
    consul_addr: @str
    redis_addr: @str

    async fn route_request(self, request):
        """Route request to appropriate microservice"""

        # Extract service from path
        let parts = request.path.split("/")
        let service_name = parts[2] if len(parts) > 2 else "default"

        # Discover service instances
        let registry = consul.connect(self.consul_addr)
        let instances = await registry.health.service(service_name, passing=true)

        if len(instances) == 0:
            return http.Response{
                status: 503,
                body: @dict{error: "Service unavailable"}
            }

        # Load balancing (round-robin)
        let instance = instances[hash(request.id) % len(instances)]
        let target_url = "http://{}:{}{}".format(
            instance.Service.Address,
            instance.Service.Port,
            request.path
        )

        # Forward request
        let response = await http.request(
            method=request.method,
            url=target_url,
            headers=request.headers,
            body=request.body
        )

        return response

    async fn authenticate(self, request):
        """Authenticate request"""

        let token = request.headers.get("Authorization")

        if not token:
            return @dict{authenticated: false}

        # Validate token (from cache or auth service)
        let cache = redis.connect(self.redis_addr)
        let cached_user = await cache.get("token:" + token)

        if cached_user:
            return @dict{
                authenticated: true,
                user: json.parse(cached_user)
            }

        # Call auth service
        let auth_response = await http.post(
            "http://auth-service/validate",
            @dict{token: token}
        )

        if auth_response.status == 200:
            # Cache result
            await cache.setex("token:" + token, 300, json.stringify(auth_response.body))

            return @dict{
                authenticated: true,
                user: auth_response.body
            }

        return @dict{authenticated: false}

    async fn rate_limit(self, request):
        """Rate limiting"""

        let user_id = request.headers.get("X-User-ID")
        let key = "ratelimit:" + user_id

        let cache = redis.connect(self.redis_addr)

        let count = await cache.incr(key)

        if count == 1:
            await cache.expire(key, 60)  # 1 minute window

        if count > 100:  # 100 requests per minute
            return @dict{allowed: false, retry_after: await cache.ttl(key)}

        return @dict{allowed: true, remaining: 100 - count}

# Start gateway
mn:
    let gateway = APIGateway{
        consul_addr: "http://consul:8500",
        redis_addr: "redis://localhost:6379"
    }

    async fn handler(request):
        # Authenticate
        let auth = await gateway.authenticate(request)
        if not auth.authenticated:
            return http.Response{status: 401, body: @dict{error: "Unauthorized"}}

        # Rate limit
        let limit = await gateway.rate_limit(request)
        if not limit.allowed:
            return http.Response{
                status: 429,
                headers: @dict{"Retry-After": str(limit.retry_after)},
                body: @dict{error: "Too many requests"}
            }

        # Route
        return await gateway.route_request(request)

    http.listen(8080, handler)
```

---

## Distributed Transactions

### Saga Pattern

```gul
@imp gul{saga, nats}

struct OrderSaga:
    """Orchestrate distributed transaction for order creation"""

    messaging: @dict

    async fn create_order(self, order_data):
        """Execute saga for order creation"""

        let saga = saga.Saga{
            name: "create-order",
            steps: @list[
                self.reserve_inventory,
                self.process_payment,
                self.create_shipment,
                self.send_confirmation
            ],
            compensations: @list[
                self.release_inventory,
                self.refund_payment,
                self.cancel_shipment,
                null
            ]
        }

        try:
            let result = await saga.execute(order_data)
            return @dict{success: true, order_id: result.order_id}
        catch error:
            print("Saga failed:", error)
            return @dict{success: false, error: str(error)}

    async fn reserve_inventory(self, data):
        """Step 1: Reserve inventory"""
        let response = await self.call_service(
            "inventory-service",
            "reserve",
            @dict{items: data.items}
        )

        if not response.success:
            raise Exception("Inventory not available")

        return @dict{reservation_id: response.reservation_id}

    async fn release_inventory(self, data):
        """Compensation: Release inventory"""
        await self.call_service(
            "inventory-service",
            "release",
            @dict{reservation_id: data.reservation_id}
        )

    async fn process_payment(self, data):
        """Step 2: Process payment"""
        let response = await self.call_service(
            "payment-service",
            "charge",
            @dict{
                amount: data.total,
               payment_method: data.payment_method
            }
        )

        if not response.success:
            raise Exception("Payment failed")

        return @dict{transaction_id: response.transaction_id}

    async fn refund_payment(self, data):
        """Compensation: Refund payment"""
        await self.call_service(
            "payment-service",
            "refund",
            @dict{transaction_id: data.transaction_id}
        )

    async fn call_service(self, service, method, data):
        """Call service via message queue"""
        let nats_client = nats.connect(self.messaging.url)

        # Request-reply pattern
        let response = await nats_client.request(
            subject="{}.{}".format(service, method),
            payload=json.stringify(data),
            timeout=5.0
        )

        return json.parse(response.data)

# Usage
mn:
    let saga = OrderSaga{
        messaging: @dict{url: "nats://localhost:4222"}
    }

    let result = await saga.create_order(@dict{
        items: @list[@dict{sku: "ABC123", quantity: 2}],
        total: 99.99,
        payment_method: "credit_card"
    })

    print("Order result:", result)
```

---

## Best Practices

### 1. Health Checks

```gul
struct HealthCheck:

    async fn @dict check(self):
        """Comprehensive health check"""

        let checks = @dict{
            database: await self.check_database(),
            cache: await self.check_cache(),
            downstream_services: await self.check_services()
        }

        let healthy = all(checks.values())

        return @dict{
            status: "healthy" if healthy else "unhealthy",
            checks: checks
        }

    async fn @bool check_database(self):
        try:
            let conn = postgres.connect("postgresql://localhost/db")
            await conn.query("SELECT 1")
            return true
        catch:
            return false
```

### 2. Distributed Tracing

```gul
@imp gul.jaeger

fn trace_request(fn):
    """Decorator for distributed tracing"""

    async fn wrapped(*args, **kwargs):
        # Start span
        let tracer = jaeger.get_tracer()
        let span = tracer.start_span("request")

        # Add metadata
        span.set_tag("service", "user-service")
        span.set_tag("version", "v1")

        try:
            let result = await fn(*args, **kwargs)
            span.set_tag("status", "success")
            return result
        catch error:
            span.set_tag("status", "error")
            span.set_tag("error", str(error))
            raise error
        finally:
            span.finish()

    return wrapped
```

### 3. Service-to-Service Authentication

```gul
struct mTLS:
    """Mutual TLS authentication"""

    cert_path: @str
    key_path: @str
    ca_path: @str

    fn configure_client(self):
        """Configure mTLS for client"""

        return grpc.ChannelCredentials{
            root_certificates: fs.read_file(self.ca_path),
            private_key: fs.read_file(self.key_path),
            certificate_chain: fs.read_file(self.cert_path)
        }
```

---

## See Also

- [Data Engineering Guide](data-engineering.md)
- [gRPC Documentation](https://grpc.io)
- [Istio Documentation](https://istio.io)

---

**Last Updated**: 2025-12-28  
**Version**: 0.13.0  
**Status**: Production Ready
