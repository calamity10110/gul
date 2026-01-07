# GUL v3.4 Design Specification: Data-Flow & Machine Learning

**Status**: Draft
**Target Version**: v3.4+
**Related**: `docs/reference/specification.md`

This document specifies the design for integrating Data-Flow and Machine Learning capabilities into GUL, building upon the v3.3 core (strong typing, ownership, Cranelift backend).

---

## 1. Feature Comparison & Synthesis

| Feature | Proposed Spec | GUL v3.3 Alignment | Implementation Strategy |
| :--- | :--- | :--- | :--- |
| **Pipeline Types** | `|>` operator | Consistent with modern functional syntax. | **Syntactic Sugar**: Lower to nested function calls in AST. |
| **Reactive Variables** | `@flow var` | Fits `@` decorator pattern. | **Compiler Transformation**: Lower to dependency graph + dirty flags. |
| **Concurrency** | `@chan<T>` | Fits `@` decorator. Requires runtime. | **Runtime Primitive**: Implementing bounded SPSC/MPMC queues in `stdlib.c`. |
| **DataFrames** | `@frame` | Parallel to `@tabl`. | **Columnar Storage**: Struct of Arrays (SoA) layout in memory. |
| **Tensors** | `@tensor<T>[Dims]` | Matches generic/collection syntax. | **Native Array**: Contiguous memory, BLAS integration. |
| **Auto-Differentiation** | `@grad` scope | Block-scoped resource management. | **Tape Generation**: Inject "tape" object into scope; operation overloading. |

---

## 2. EBNF Grammar Specification

This grammar extends the core GUL v3.3 grammar.

```ebnf
/* extended_expression */
expression ::= pipeline_expression 
             | tensor_literal 
             | frame_literal 
             | ... (existing expressions)

/* 1. Pipeline Operator */
pipeline_expression ::= term { "|>" function_call_or_lambda }
function_call_or_lambda ::= function_call | lambda_expression

/* 2. Flow Variables */
variable_decl ::= "@flow" "var" identifier ":" type "=" expression

/* 3. Channels */
channel_type ::= "@chan" "<" type ">"
channel_ctor ::= "@chan" "<" type ">" "(" "capacity" "=" integer ")"

/* 4. Tensor Literals & Types */
tensor_type ::= "@tensor" "<" type ">" "[" dimensions "]"
dimensions ::= integer { "," integer }
tensor_literal ::= "@tensor" [ "<" type ">" ] [ "[" dimensions "]" ] "{" tensor_data "}"
tensor_data ::= number { "," number } | "{" tensor_data "}" { "," "{" tensor_data "}" }

/* 5. DataFrame Literals */
frame_literal ::= "@frame" "{" 
                    "columns" ":" "(" string_list ")" "," 
                    "data" ":" row_data 
                  "}"
row_data ::= identifier ":" "{" value_list "}" { "," identifier ":" "{" value_list "}" }

/* 6. Gradient Scope */
grad_scope ::= "@grad" ":" block
```

---

## 3. IR Lowering Rules

### 3.1 Pipelining (`|>`)

**Source:**

```gul
data |> filter(p) |> map(f)
```

**AST Transformation (Desugaring):**

```gul
map(filter(data, p), f)
```

**Type Rules:**

1. Let `data` be of type `T1`.
2. `filter` must accept `(T1, Predicate) -> T2`.
3. `map` must accept `(T2, Function) -> T3`.

### 3.2 Flow Variables (`@flow`)

**Source:**

```gul
@flow var x = 10
@flow var y = x * 2
```

**Lowered IR (Pseudocode):**

```rust
struct FlowVariable<T> {
    value: T,
    subscribers: List<FlowCallback>
}

let x = FlowVariable { value: 10, ... }
let y = FlowVariable { value: x.value * 2, ... }

// Dependency Registration
x.subscribe(() => {
    y.value = x.value * 2;
    y.notify_subscribers();
})
```

### 3.3 Tensors & Gradients

**Source:**

```gul
@grad:
    let y = matmul(w, x)
```

**Lowered IR:**

```rust
// Auto-generated Context
let _tape = gul_autograd_begin()

// Operation Lowering
let y = gul_tensor_matmul_with_grad(w, x, _tape)

gul_autograd_end(_tape)
```

---

## 4. Standard Library Design

### 4.1 `std.flow`

Reactive primitives.

* **`Channel<T>`**: Typed message passing.
  * `send(val: move T)`
  * `recv() -> T`
  * `try_recv() -> Option<T>`
* **`Signal<T>`**: Behavioral values (flow variables).

### 4.2 `std.frame`

DataFrame operations (Column-major).

* **`DataFrame` type**:
  * `filter(predicate)` -> `DataFrame` (Zero-copy view)
  * `select(cols...)` -> `DataFrame`
  * `group_by(col)` -> `GroupedFrame`
  * `join(other, on, how)` -> `DataFrame`

### 4.3 `std.ml`

Machine Learning core.

* **`Tensor<T, D>`**:
  * `reshape(dims)`
  * `transpose(dims)`
  * `to(device)` (CPU/GPU)
* **`Optimizer`**:
  * `SGD(params, lr)`
  * `Adam(params, lr)`
* **`Loss`**:
  * `mse(pred, target)`
  * `cross_entropy(pred, target)`

---

## 5. Example Programs

### Example 1: End-to-End ML Pipeline

```gul
@imp std.ml
@imp std.frame

# 1. Load Data
let df = @frame {
    columns: ("feature", "label"),
    data:
        r1: {1.0, 0.0},
        r2: {2.5, 1.0}
} 

# 2. Pipeline Processing
let features = df 
    |> df.filter(r => r.feature > 0)
    |> df.select("feature")
    |> ml.to_tensor()

# 3. Model Definition
struct Model:
    weight: @tensor<float>[1, 1]
    bias: float

    @fn forward(self, x):
        return ml.matmul(x, self.weight) + self.bias

# 4. Training
let m = Model{
    weight: @tensor<float>[1.0], 
    bias: 0.0
}
let opt = ml.SGD(m, lr=0.01)

for epoch in 0..10:
    @grad:
        let pred = m.forward(features)
        let loss = ml.mse(pred, 0.5) # Dummy target
        loss.backward()
    opt.step()
```

---

## 6. Compiler Milestone Plan

### Phase 1: Syntactic Sugar (v3.4)

* **Goal**: Enable pipeline syntax and literal parsing.
* **Tasks**:
    1. Update Lexer/Parser for `|>` operator.
    2. Update Parser for `@frame` and `@tensor` literals.
    3. Implement AST desugaring for `|>`.

### Phase 2: Core Runtime (v3.5)

* **Goal**: Native memory layout for Tensors and Frames.
* **Tasks**:
    1. Implement `gul_tensor_*` in `stdlib.c`.
    2. Implement `gul_frame_*` (columnar store) in `stdlib.c`.
    3. Bind intrinsics in Cranelift backend.

### Phase 3: Flow & Concurrency (v3.6)

* **Goal**: Channels and Reactive variables.
* **Tasks**:
    1. Implement `@chan` type checking.
    2. Add thread/coroutine support in runtime.
    3. Implement `@flow` dependency analysis in Semantic Analyzer.

### Phase 4: Autograd Engine (v3.7)

* **Goal**: Differentiable programming.
* **Tasks**:
    1. Implement Tape machine in `stdlib.c`.
    2. Add operator overloading for `@grad` scope.
    3. Implement backward pass primitives.
