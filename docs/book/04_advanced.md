# Chapter 4: Advanced Concepts

## Concurrency

GUL natively supports async/await. For parallel execution, use `std.async`.

### Spawning Tasks

You can run tasks in parallel using `std.async.spawn` to create a `Task`, and `std.async.join` (or `join_all`) to await them.

```gul
@imp std.async

@fn worker(id: int):
    println("Worker " + id + " starting")
    std.async.sleep(100) # Simulating work
    println("Worker " + id + " done")

mn:
    let task1 = std.async.spawn(worker(1))
    let task2 = std.async.spawn(worker(2))

    std.async.join(task1)
    std.async.join(task2)
    println("All done")
```

## Foreign Function Interface (FFI)

GUL can seamlessly interoperate with C, Rust, and Python.

### Rust Block

You can embed Rust code directly using `@rust`:

```gul
@rust {
    @fn fast_calculate(x: int) -> int {
        return x * x;
    }
}
```

### Python Block

Use Python for quick scripting or library access:

```gul
@python {
    def plot_data(data):
        import matplotlib.pyplot as plt
        plt.plot(data)
}
```

## AI Integration

GUL treats AI models as first-class citizens.

```gul
@ai(model="gpt-4", temperature=0.7)
@fn generate_story(prompt: str) -> str

mn:
    let story = await generate_story("Once upon a time")
    println(story)
```
