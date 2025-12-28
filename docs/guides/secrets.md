# Secrets

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# Secret Management Guide

GUL provides built-in secret management capabilities to handle sensitive data like API keys, passwords, database credentials, and other secrets securely. This guide covers best practices and usage patterns for secret management in GUL.

## 🔐 Overview

Secret management in GUL follows these principles:

1. **Never store secrets in code**: Use environment variables or secure vaults
2. **Encrypt secrets at rest**: Secrets are encrypted when stored
3. **Audit secret access**: Track when and where secrets are accessed
4. **Rotate secrets regularly**: Support for secret rotation
5. **Scope secrets appropriately**: Limit secret access to necessary components

## 🎯 Secret Types

GUL supports several types of secrets:

### API Keys

```gul
import std.secrets

secret api_key: str = secrets.get("OPENAI_API_KEY")

# Use in your code
response = http.get("https://api.openai.com/v1/models", {
    headers: {
        "Authorization": f"Bearer {api_key}"
    }
})
```

### Database Credentials

```gul
import std.secrets
import std.database

secret db_config = secrets.get_struct("database", {
    host: str,
    port: int,
    username: str,
    password: str,
    database: str
})

# Connect using secrets
db = database.connect(
    f"postgresql://{db_config.username}:{db_config.password}@{db_config.host}:{db_config.port}/{db_config.database}"
)
```

### OAuth Tokens

```gul
import std.secrets

secret oauth = secrets.get_oauth("github", {
    client_id: str,
    client_secret: str,
    access_token: str,
    refresh_token: str
})

# Use with auto-refresh
api_response = http.get("https://api.github.com/user", {
    headers: {
        "Authorization": f"token {oauth.access_token}"
    }
})
```

### Encryption Keys

```gul
import std.secrets
import std.crypto

secret encryption_key = secrets.get_key("aes_master_key", "aes256")

# Encrypt sensitive data
encrypted_data = crypto.aes_encrypt(user_data, encryption_key)
```

## 📝 Declaring Secrets

### Basic Secret Declaration

```gul
# Simple secret
secret api_key: str

# Secret with default from environment
secret api_key: str = env("API_KEY")

# Secret with validation
secret api_key: str @validate(length > 20)

# Optional secret
secret optional_key: str? = None
```

### Structured Secrets

```gul
# Secret struct
secret db_credentials: struct {
    host: str
    port: int = 5432
    username: str
    password: str @validate(length >= 12)
    ssl: bool = True
}

# Access structured secret fields
print(f"Connecting to {db_credentials.host}:{db_credentials.port}")
```

### Secret Collections

```gul
# Multiple API keys
secret api_keys: map[str, str] = {
    "openai": env("OPENAI_KEY"),
    "anthropic": env("ANTHROPIC_KEY"),
    "google": env("GOOGLE_KEY")
}

# List of secrets
secret allowed_ips: list[str] = [
    env("ADMIN_IP_1"),
    env("ADMIN_IP_2")
]
```

## 🌍 Environment Variables

### Loading from Environment

```gul
import std.secrets

# Load single environment variable
secret api_key = secrets.from_env("API_KEY")

# Load with default value
secret api_key = secrets.from_env("API_KEY", default="development-key")

# Load with type conversion
secret port: int = secrets.from_env("PORT", type=int, default=8080)

# Required environment variable (fails if missing)
secret critical_key = secrets.require_env("CRITICAL_API_KEY")
```

### Environment Files (.env)

Create a `.env` file in your project root:

```bash
# .env
API_KEY=sk-1234567890abcdef
DATABASE_URL=postgresql://user:pass@localhost:5432/mydb
REDIS_URL=redis://localhost:6379
DEBUG=true
```

Load in GUL:

```gul
import std.secrets

# Load all environment variables from .env
secrets.load_env(".env")

# Access loaded secrets
secret api_key = env("API_KEY")
secret database_url = env("DATABASE_URL")
```

### Multiple Environments

```gul
import std.secrets

# Load environment-specific configuration
environment = env("ENVIRONMENT", default="development")

match environment:
    "production":
        secrets.load_env(".env.production")
    "staging":
        secrets.load_env(".env.staging")
    "development":
        secrets.load_env(".env.development")
    _:
        secrets.load_env(".env.local")
```

## 🔒 Secret Vaults

### HashiCorp Vault Integration

```gul
import std.secrets.vault

# Initialize Vault connection
vault = vault.connect(
    url="https://vault.example.com:8200",
    token=env("VAULT_TOKEN")
)

# Read secret from Vault
secret db_password = vault.read("secret/data/database/password")

# Write secret to Vault
vault.write("secret/data/api/key", {
    "key": "sk-1234567890",
    "created": "2025-12-28"
})

# Dynamic database credentials
db_creds = vault.database.dynamic("postgresql", role="readonly")
# Credentials are automatically rotated
```

### AWS Secrets Manager

```gul
import std.secrets.aws

# Initialize AWS Secrets Manager
aws_secrets = aws.SecretsManager(
    region="us-east-1",
    access_key=env("AWS_ACCESS_KEY_ID"),
    secret_key=env("AWS_SECRET_ACCESS_KEY")
)

# Get secret from AWS
secret api_key = aws_secrets.get("production/api/openai")

# Get secret with automatic JSON parsing
secret db_config = aws_secrets.get_json("production/database/config")
```

### Azure Key Vault

```gul
import std.secrets.azure

# Initialize Azure Key Vault
keyvault = azure.KeyVault(
    vault_url="https://myvault.vault.azure.net",
    credential=azure.DefaultAzureCredential()
)

# Get secret
secret api_key = keyvault.get_secret("api-key")

# Get certificate
cert = keyvault.get_certificate("tls-cert")
```

### Google Secret Manager

```gul
import std.secrets.google

# Initialize Google Secret Manager
gsm = google.SecretManager(
    project_id="my-project",
    credentials=env("GOOGLE_APPLICATION_CREDENTIALS")
)

# Access secret
secret api_key = gsm.access("projects/my-project/secrets/api-key/versions/latest")
```

## 🔐 Encryption

### Encrypting Secrets at Rest

```gul
import std.secrets
import std.crypto

# Define master encryption key
secret master_key = secrets.get_key("master_encryption_key", "aes256")

# Encrypt sensitive data before storage
sensitive_data = "user-credit-card-number"
encrypted = crypto.aes_encrypt(sensitive_data, master_key)

# Store encrypted data
database.insert("users", {
    id: user_id,
    payment_info: encrypted
})

# Decrypt when needed
encrypted_data = database.query("SELECT payment_info FROM users WHERE id = ?", [user_id])
decrypted = crypto.aes_decrypt(encrypted_data, master_key)
```

### In-Memory Encryption

```gul
import std.secrets

# Secrets are automatically scrubbed from memory when no longer needed
fn process_payment(secret card_number: str):
    # card_number is secured in memory
    payment_processor.charge(card_number, amount)
    # card_number is automatically zeroed out after function returns
```

## 🔄 Secret Rotation

### Manual Rotation

```gul
import std.secrets

# Rotate API key
old_key = secrets.get("API_KEY")
new_key = generate_new_api_key()

# Update in all systems
secrets.set("API_KEY", new_key)
secrets.set("API_KEY_OLD", old_key)  # Keep old key for rollback

# Notify services
notify_services_of_key_rotation(new_key)

# Remove old key after grace period
secrets.delete("API_KEY_OLD")
```

### Automatic Rotation

```gul
import std.secrets.rotation

# Define rotation policy
rotation_policy = rotation.Policy({
    secret_name: "database_password",
    rotate_every: duration.days(30),
    rotation_function: rotate_db_password,
    notify_on_rotation: ["admin@example.com"]
})

# Register rotation policy
secrets.register_rotation(rotation_policy)

# Rotation function
fn rotate_db_password(old_password: str): str:
    new_password = generate_secure_password(length=32)

    # Update database
    database.admin_query(
        "ALTER USER app_user WITH PASSWORD ?",
        [new_password]
    )

    # Update vault
    vault.write("secret/database/password", new_password)

    return new_password
```

## 🔍 Audit Logging

### Enable Secret Access Logging

```gul
import std.secrets

# Configure audit logging
secrets.configure_audit({
    enabled: True,
    log_location: "/var/log/gul/secrets-audit.log",
    log_format: "json",
    include_access: True,
    include_failures: True
})

# Access secrets (automatically logged)
secret api_key = secrets.get("API_KEY")
# Logs: {"timestamp": "2025-12-28T13:45:00Z", "secret": "API_KEY", "action": "read", "user": "app", "result": "success"}
```

### Query Audit Logs

```gul
import std.secrets.audit

# Get audit log entries
logs = audit.query({
    secret_name: "API_KEY",
    start_time: datetime.now() - duration.days(7),
    end_time: datetime.now()
})

for log in logs:
    print(f"{log.timestamp}: {log.action} by {log.user} - {log.result}")
```

## 🚨 Security Best Practices

### 1. Never Hardcode Secrets

❌ **Bad:**

```gul
api_key = "sk-1234567890abcdef"  # Never do this!
```

✅ **Good:**

```gul
secret api_key = env("API_KEY")
```

### 2. Use Strong Validation

```gul
secret api_key: str
    @validate(length >= 20)
    @validate(regex="^sk-[a-zA-Z0-9]+$")
    @required

secret password: str
    @validate(length >= 12)
    @validate(has_uppercase=True)
    @validate(has_number=True)
    @validate(has_special=True)
```

### 3. Limit Secret Scope

```gul
# Scope secrets to specific modules
@module(secrets=["API_KEY"])
module payment_processing:
    secret api_key = secrets.get("API_KEY")
    # api_key only accessible within this module
```

### 4. Use Secret Expiration

```gul
secret temp_token = secrets.create_temporary(
    value=generate_token(),
    expires_in=duration.hours(24)
)
```

### 5. Implement Secret Masking

```gul
import std.secrets

# Secrets are automatically masked in logs
secret api_key = env("API_KEY")
print(f"Using API key: {api_key}")
# Output: "Using API key: sk-****...***f"
```

## 🧪 Testing with Secrets

### Mock Secrets in Tests

```gul
import std.testing
import std.secrets

@test
fn test_api_integration():
    # Use test secrets
    testing.mock_secret("API_KEY", "test-key-12345")

    # Your test code
    result = call_api()

    assert result.status_code == 200
```

### Test Environment Setup

```gul
# tests/.env.test
API_KEY=test-api-key
DATABASE_URL=sqlite::memory:
DEBUG=true
```

## 📊 Secret Management CLI

### Command-Line Operations

```bash
# List all secrets
gul secrets list

# Get a secret value (masked)
gul secrets get API_KEY

# Set a secret
gul secrets set API_KEY sk-1234567890abcdef

# Delete a secret
gul secrets delete OLD_API_KEY

# Encrypt a file
gul secrets encrypt secrets.json

# Decrypt a file
gul secrets decrypt secrets.json.enc

# Rotate a secret
gul secrets rotate DATABASE_PASSWORD

# Validate secrets
gul secrets validate
```

## 🔗 Integration Examples

### Complete Web Application Example

```gul
import std.http
import std.database
import std.secrets

# Load environment
secrets.load_env(".env")

# Database secrets
secret db_config = secrets.get_struct("database", {
    host: env("DB_HOST"),
    port: env("DB_PORT", type=int),
    username: env("DB_USER"),
    password: env("DB_PASSWORD"),
    database: env("DB_NAME")
})

# API secrets
secret api_keys = {
    "stripe": env("STRIPE_SECRET_KEY"),
    "sendgrid": env("SENDGRID_API_KEY"),
    "aws": env("AWS_SECRET_KEY")
}

# Application secret
secret jwt_secret = env("JWT_SECRET")

# Initialize database
db = database.connect(
    f"postgresql://{db_config.username}:{db_config.password}@{db_config.host}:{db_config.port}/{db_config.database}"
)

# HTTP server with secrets
server = http.Server(port=8080)

@server.route("/api/payment")
fn process_payment(request):
    # Use Stripe secret
    stripe.charge(
        api_key=api_keys["stripe"],
        amount=request.json["amount"]
    )
```

## 📚 See Also

- [Web Development Guide](web-development.md)
- [Database Module](../api/database.md)
- [HTTP Module](../api/http.md)
- [Testing & Deployment](testing-deployment.md)

---

**Last Updated**: 2025-12-28  
**Version: 0.13.0  
**License**: MIT
