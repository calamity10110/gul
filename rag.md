# GUL Language RAG

GUL is a modern, multi-paradigm programming language designed for simplicity, performance, and interoperability.

## Gul Language Syntax

### Imports
```gul
@imp std.io
@imp std{http, json}
@imp python{pandas, numpy}
```

### Variables & Data Structures
```gul
let name: str = "Gul" // Immutable
var count: int = 1 // Mutable

let my_list = @list[1, 2, 3]
let my_dict = @dict{
    "key": "value"
}
```

### Control Flow
```gul
if count > 0:
    print("Positive")
elif count == 0:
    print("Zero")
else:
    print("Negative")

for i in 0..5:
    print(i)
```

### Functions
```gul
fn add(a: int, b: int) -> int:
    return a + b

// Arrow function
let multiply = (a, b) => a * b
```

### `mn:` Structure
The `mn:` block is the main entry point for a Gul program.
```gul
mn:
    print("Hello from mn:")
```

### Foreign Code Integration
Embed other languages directly in Gul.
```gul
@python {
    import numpy as np
    a = np.array([1, 2, 3])
    print(a)
}

@rust {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

## Example Applications

### Hello World
```gul
mn:
    print("Hello, World!")
```

### Simple Web Server
```gul
@imp std.http

fn handler(req):
    return @dict{
        "status": 200,
        "body": "Hello from Gul web server!"
    }

mn:
    http.listen(8080, handler)
```

### Data Processing with Python
```gul
@imp python{pandas}

@python {
    data = {'name': ['Alice', 'Bob'], 'age': [25, 30]}
    df = pandas.DataFrame(data)
    print(df.describe())
}

mn:
    print("Python data processing complete.")
```

## Package Data Dictionary

(std.io : (print - print to console), (println - print to console with newline), (input - read a line from console), (input_int - read an integer from console))
(std.http : (get - make a GET request), (post - make a POST request), (listen - start a web server))
(std.fs : (read_file - read file contents), (write_file - write to a file), (append_file - append to a file), (list_dir - list directory contents))
(std.math : (sqrt - square root), (pow - power), (abs - absolute value), (sin - sine), (cos - cosine))
