// Enhanced code generation functions for GUL v3.2 MCP
// Generates proper v3.2 syntax with full annotations


/// Generate code based on description with v3.2 syntax
pub fn generate_v32_code(description: &str, _use_ai: bool) -> Result<String, String> {
    // Use template-based generation
    Ok(generate_from_template(description))
}

fn generate_from_template(description: &str) -> String {
    let desc_lower = description.to_lowercase();
    
    if desc_lower.contains("api") || desc_lower.contains("web") || desc_lower.contains("rest") {
        generate_web_api_template(description)
    } else if desc_lower.contains("data") || desc_lower.contains("analysis") || desc_lower.contains("csv") {
        generate_data_template(description)
    } else if desc_lower.contains("embedded") || desc_lower.contains("gpio") || desc_lower.contains("led") {
        generate_embedded_template(description)
    } else {
        generate_generic_template(description)
    }
}

fn generate_web_api_template(description: &str) -> String {
    format!(r#"# GUL v3.2 Web API - Generated Code
# Description: {}
# Syntax: v3.2 with full type annotations and ownership modes

@imp std.http
@imp std.json
@imp std.db

# Type definition with @ prefix constructors
struct User:
    id: @int
    username: @str
    email: @str
    created_at: @str

# Async function with full type annotations
async @dict get_users() -> @dict:
    """Fetch all users from database"""
    let users = await db.query("SELECT * FROM users ORDER BY created_at DESC")
    return @dict{{
        status: @int(200),
        data: users
    }}

async @dict create_user(req) -> @dict:
    """Create new user with validation"""
    # Extract data from request
    let username = @str(req.body["username"])
    let email = @str(req.body["email"])
    
    # Validate input (borrow for read-only access)
    if not validate_email(borrow email):
        return @dict{{
            status: @int(400),
            error: @str("Invalid email format")
        }}
    
    # Insert into database
    let user_id = await db.execute(
        "INSERT INTO users (username, email) VALUES (?, ?)",
        username, email
    )
    
    return @dict{{
        status: @int(201),
        id: user_id,
        message: @str("User created successfully")
    }}

fn @bool validate_email(email: borrow @str) -> @bool:
    """Validate email format (uses borrow for efficiency)"""
    return email.contains("@") && email.contains(".")

# Main entry point
mn:
    print(@str("🚀 Starting GUL v3.2 Web API Server..."))
    
    # Configure routes
    http.get("/api/users", get_users)
    http.post("/api/users", create_user)
    
    # Start server with type-annotated port
    let port = @int(8080)
    print(@str(f"✅ Server running on http://localhost:{{port}}"))
    http.listen(port)
"#, description)
}

fn generate_data_template(description: &str) -> String {
    format!(r#"# GUL v3.2 Data Analysis - Generated Code
# Description: {}
# Syntax: v3.2 with Python integration and type annotations

@imp std.io
@imp std.json
@imp python{{pandas, numpy, matplotlib}}

# Type-annotated data analysis struct
struct DataAnalyzer:
    filepath: @str
    results: @dict
    
    fn @dict analyze(self) -> @dict:
        """Perform comprehensive data analysis"""
        
        @python {{
            import pandas as pd
            import numpy as np
            import matplotlib.pyplot as plt
            
            # Load and analyze data
            df = pd.read_csv(self.filepath)
            
            # Statistical analysis with type conversion
            stats = {{
                "count": int(len(df)),
                "mean": float(df.mean().mean()),
                "median": float(df.median().median()),
                "std": float(df.std().mean()),
                "columns": list(df.columns),
                "dtypes": {{col: str(dtype) for col, dtype in df.dtypes.items()}}
            }}
            
            # Generate visualization
            plt.figure(figsize=(12, 6))
            df.plot(kind='bar')
            plt.title('Data Analysis Results')
            plt.tight_layout()
            plt.savefig('analysis_output.png', dpi=300)
            plt.close()
            
            analysis_results = stats
        }}
        
        # Return with type annotation
        return @dict(python.analysis_results)
    
    fn @str export_json(self) -> @str:
        """Export results to JSON string"""
        return json.stringify(self.results)

fn @list load_csv(filepath: borrow @str) -> @list:
    """Load CSV file efficiently (borrow to avoid copy)"""
    let content = io.read_file(filepath)
    let lines = content.split("\\n")
    
    var data = @list[]
    
    for line in lines:
        if line.trim():
            let fields = line.split(",")
            data.append(@list(fields))
    
    return data

# Main entry point with full annotations
mn:
    print(@str("📊 GUL v3.2 Data Analysis Pipeline"))
    print(@str("=" * 50))
    
    # Create analyzer with type constructors
    let analyzer = DataAnalyzer{{
        filepath: @str("data.csv"),
        results: @dict{{}}
    }}
    
    # Run analysis
    print(@str("\\nAnalyzing data..."))
    let results = analyzer.analyze()
    
    # Display results with pretty printing
    print(@str("\\n✅ Analysis Complete!"))
    for key, value in results.items():
        print(@str(f"  {{key}}: {{value}}"))
    
    # Export results
    let json_output = analyzer.export_json()
    io.write_file("results.json", json_output)
    
    print(@str("\\n💾 Results saved:"))
    print(@str("  - results.json"))
    print(@str("  - analysis_output.png"))
"#, description)
}

fn generate_embedded_template(description: &str) -> String {
    format!(r#"# GUL v3.2 Embedded System - Generated Code
# Description: {}
# Syntax: v3.2 for ESP32/RP2040 with full annotations

@imp embedded.gpio
@imp embedded.time
@imp embedded.pwm

# Device configuration with type annotations
struct DeviceConfig:
    led_pin: @int
    button_pin: @int
    pwm_frequency: @int

# Device controller with ownership annotations
struct Device:
    config: DeviceConfig
    led: gpio.Pin
    button: gpio.Pin
    
    fn initialize(ref self):
        """Initialize GPIO pins (mutable reference for setup)"""
        self.led = gpio.pin(self.config.led_pin, OUTPUT)
        self.button = gpio.pin(self.config.button_pin, INPUT_PULLUP)
        print(@str("✅ GPIO initialized"))
    
    fn @bool is_button_pressed(self) -> @bool:
        """Check button state (immutable self)"""
        return self.button.read() == LOW
    
    fn toggle_led(ref self):
        """Toggle LED (requires mutable reference)"""
        self.led.toggle()
    
    fn set_brightness(self, level: @int):
        """Set LED brightness via PWM"""
        pwm.set_duty(self.config.pwm_frequency, level)

fn @int map_range(value: @int, from_low: @int, from_high: @int, 
                  to_low: @int, to_high: @int) -> @int:
    """Map value from one range to another"""
    let from_range = from_high - from_low
    let to_range = to_high - to_low
    return (value - from_low) * to_range / from_range + to_low

# Main entry point
mn:
    print(@str("🔧 GUL v3.2 Embedded System Starting..."))
    
    # Configure with type constructors
    let config = DeviceConfig{{
        led_pin: @int(2),
        button_pin: @int(0),
        pwm_frequency: @int(1000)
    }}
    
    # Initialize device
    var device = Device{{
        config: config,
        led: gpio.pin(@int(0), OUTPUT),
        button: gpio.pin(@int(0), INPUT)
    }}
    
    device.initialize()
    
    # Main control loop
    var brightness = @int(0)
    var direction = @int(1)
    var last_button_state = @bool(false)
    
    print(@str("🎮 System ready - Press button to toggle"))
    
    loop:
        # Button handling with debounce
        let button_pressed = device.is_button_pressed()
        
        if button_pressed && not last_button_state:
            device.toggle_led()
            print(@str("LED toggled"))
            time.delay_ms(@int(200))
        
        last_button_state = button_pressed
        
        # Breathing LED effect
        device.set_brightness(brightness)
        brightness = brightness + (direction * @int(5))
        
        # Reverse direction at limits
        if brightness >= @int(255):
            brightness = @int(255)
            direction = @int(-1)
        elif brightness <= @int(0):
            brightness = @int(0)
            direction = @int(1)
        
        time.delay_ms(@int(10))
"#, description)
}

fn generate_generic_template(description: &str) -> String {
    format!(r#"# GUL v3.2 Application - Generated Code
# Description: {}
# Syntax: v3.2 with full type annotations and ownership modes

@imp std.io
@imp std.collections
@imp std.json

# Type-annotated struct
struct AppConfig:
    name: @str
    version: @str
    debug: @bool
    port: @int

# Immutable function with borrow
fn @str format_log(message: borrow @str, level: @str) -> @str:
    """Format log message (borrow to avoid copy)"""
    let timestamp = time.now()
    return @str(f"[{{timestamp}}] [{{level}}] {{message}}")

# Mutable function with ref
fn process_items(items: ref @list):
    """Process list in-place (mutable reference)"""
    items.sort()
    items.append(@str("processed"))

# Move semantics example
fn consume_data(data: move @dict) -> @str:
    """Consume data and return summary (move ownership)"""
    let keys = data.keys()
    return @str(f"Processed {{len(keys)}} keys")

# Async function with full annotations
async @dict fetch_config(url: @str) -> @dict:
    """Fetch configuration from remote source"""
    let response = await http.get(url)
    return @dict(response.json())

# Main entry point with comprehensive examples
mn:
    print(@str("🚀 GUL v3.2 Application"))
    print(@str("=" * 50))
    
    # Type constructors for all data
    let config = AppConfig{{
        name: @str("My GUL App"),
        version: @str("1.0.0"),
        debug: @bool(true),
        port: @int(8080)
    }}
    
    # Collection types with @ prefix
    var items = @list[@str("apple"), @str("banana"), @str("cherry")]
    let settings = @dict{{
        "mode": @str("production"),
        "timeout": @int(30),
        "retry": @bool(true)
    }}
    
    # Ownership examples
    let log_msg = format_log(borrow "Application started", @str("INFO"))
    print(log_msg)
    
    process_items(ref items)  # Mutable reference
    print(@str(f"Processed items: {{items}}"))
    
    # Type-safe operations
    let data = @dict{{
        "key1": @int(100),
        "key2": @str("value"),
        "key3": @bool(true)
    }}
    
    let summary = consume_data(move data)  # Move ownership
    print(summary)
    
    # Async operations
    print(@str("\\nFetching remote configuration..."))
    let remote_config = await fetch_config(@str("https://api.example.com/config"))
    
    print(@str("\\n✅ Application initialized"))
    print(@str(f"   Name: {{config.name}}"))
    print(@str(f"   Version: {{config.version}}"))
    print(@str(f"   Port: {{config.port}}"))
"#, description)
}

#[allow(dead_code)]
fn main() {
    eprintln!("Error: This module is not meant to be run directly.");
    eprintln!("Use 'gul-mcp generate <description>' instead.");
    std::process::exit(1);
}
