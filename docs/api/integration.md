# Integration

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# Multi-language Integration

The language supports seamless integration with multiple programming languages through the `cs` (custom) block syntax.

## Rust Integration

### Syntax

```
@rust {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn fast_sum(data: &[i32]) -> i32 {
        data.iter().sum()
    }
}
```

### Features

- **Zero-copy** data sharing
- **Native performance**
- Direct memory access
- Ownership model compatibility (`borrow`, `move`)

### Example

```
@imp std.io

@rust {
    fn process_image(pixels: &mut [u8], width: usize, height: usize) {
        for i in 0..pixels.len() {
            pixels[i] = pixels[i].saturating_add(10);
        }
    }
}

mn:
    let image = load_image("photo.png")
    rust.process_image(image.pixels, image.width, image.height)
    save_image(image, "output.png")
```

## Python Integration

### Syntax

```
@python {
    import numpy as np
    import pandas as pd

    def analyze_data(data):
        df = pd.DataFrame(data)
        return df.describe().to_dict()
}
```

### Features

- Access to Python ecosystem
- NumPy, Pandas, SciPy, etc.
- ML libraries (TensorFlow, PyTorch)

### Example

```
@python {
    import tensorflow as tf

    def predict(model_path, input_data):
        model = tf.keras.models.load_model(model_path)
        return model.predict(input_data).tolist()
}

@async run_inference():
    let data = load_data("input.csv")
    let result = python.predict("model.h5", data)
    print(result)
```

## JavaScript Integration

### Syntax

```
@js {
    export function formatDate(timestamp) {
        return new Date(timestamp).toLocaleDateString();
    }

    export async function fetchAPI(url) {
        const response = await fetch(url);
        return await response.json();
    }
}
```

### Features

- Full ES6+ support
- Async/await compatible
- DOM access (web targets)
- Node.js APIs

### Example

```
@js {
    export function processJSON(data) {
        return data.map(item => ({
            ...item,
            processed: true,
            timestamp: Date.now()
        }));
    }
}

mn:
    let data = load_json("data.json")
    let processed = js.processJSON(data)
    save_json(processed, "output.json")
```

## TypeScript Integration

### Syntax

```
@cs ts:
    interface User {
        id: number;
        name: string;
        email: string;
    }

    export function validateUser(user: User): boolean {
        return user.email.includes('@');
    }

    export function getUsers(): User[] {
        return [
            { id: 1, name: "Alice", email: "alice@example.com" },
            { id: 2, name: "Bob", email: "bob@example.com" }
        ];
    }
```

### Features

- Type safety
- Interface definitions
- Generics support
- Transpiles to JavaScript

### Example

```
@cs ts:
    interface Config {
        apiKey: string;
        endpoint: string;
        timeout: number;
    }

    export function createClient(config: Config) {
        return {
            get: async (path: string) => {
                const url = `${config.endpoint}${path}`;
                const response = await fetch(url, {
                    headers: { 'Authorization': `Bearer ${config.apiKey}` }
                });
                await response.json();
            }
        };
    }

async mn:
    config = {
        apiKey: secret.API_KEY,
        endpoint: "https://api.example.com",
        timeout: 5000
    }
    client = ts.createClient(config)
    data = await client.get("/users")
    print(data)
```

## C Integration

### Syntax

```
@c {
    #include <math.h>

    double calculate_distance(double x1, double y1, double x2, double y2) {
        double dx = x2 - x1;
        double dy = y2 - y1;
        return sqrt(dx*dx + dy*dy);
    }

    int fibonacci(int n) {
        return n <= 1 ? n : fibonacci(n-1) + fibonacci(n-2);
    }
}
```

### Features

- Direct C library access
- Hardware-level control
- Embedded system support
- Pointer manipulation

### Example

```
@cs c:
    #include <string.h>

    void reverse_string(char* str) {
        int len = strlen(str);
        for (int i = 0; i < len/2; i++) {
            char temp = str[i];
            str[i] = str[len-1-i];
            str[len-1-i] = temp;
        }
    }

mn:
    let text = "Hello, World!"
    c.reverse_string(ref text)
    print(text)  # "!dlroW ,olleH"
```

## SQL Integration

### Syntax

```
@sql {
    select
        users.name,
        users.email,
        count(orders.id) as order_count
    from users
    left join orders on users.id = orders.user_id
    where users.active = true
    group by users.id
    order by order_count desc
    limit 10;
}
```

### Features

- Embedded SQL engine
- Query optimization
- Type-safe results
- Transaction support

### Example

```
@imp db

@cs sql:
    create table if not exists users (
        id integer primary key,
        name text not null,
        email text unique not null,
        created_at timestamp default current_timestamp
    );

    insert into users (name, email) values
        ('Alice', 'alice@example.com'),
        ('Bob', 'bob@example.com');

mn:
    db.execute(sql.create_table)
    db.execute(sql.insert_users)

    results = db.query("select * from users where name like 'A%'")
    for row in results:
        print(row.name, row.email)
```

## Cross-Language Data Sharing

### Ownership Rules

```
# Pass by reference (borrow)
rust.process(borrow data)
python.analyze(borrow data)

# Pass by ownership (move)
rust.consume(move data)

# Pass by copy (explicit)
js.process(kept data)
```

### Type Mapping

| Universal | Rust    | Python | JS/TS   | C      |
| --------- | ------- | ------ | ------- | ------ |
| int       | i32     | int    | number  | int    |
| float     | f64     | float  | number  | double |
| str       | String  | str    | string  | char\* |
| bool      | bool    | bool   | boolean | bool   |
| list      | Vec<T>  | list   | Array   | T\*    |
| dict      | HashMap | dict   | Object  | struct |

## Performance Considerations

- **Rust**: Zero-copy, native performance
- **C**: Direct FFI, minimal overhead
- **Python**: GIL considerations, best for I/O-bound tasks
- **JS/TS**: V8 engine, good for async operations
- **SQL**: Query optimization, indexed access

## Error Handling

```
try:
    result = python.risky_operation(data)
catch error:
    print("Python error:", error)
    fallback_result = rust.safe_operation(data)
```
