# GUL Language 101

This document defines the **complete concrete syntax**, **semantic rules**, **ownership model**, and **execution model** of the GUL programming language.

GUL is a **multi-syntax, single-semantics language**.
All surface syntax lowers into a **typed, ownership-checked data-flow graph**.

---

## 1. Core Principles

1. Everything compiles to a **data-flow graph**
2. Nodes own data, edges describe access or transfer
3. Ownership is explicit and statically checked
4. No unused outputs are allowed
5. Execution order is derived, not written

---

## 2. Values, Ownership, and Edges

### 2.1 Ownership

```
own<T>
```

- Exactly one owner exists at a time
- Ownership exists only inside a node
- Ownership does **not** imply mutability or sharing

---

### 2.2 Edge Access Modes

Edges define how downstream nodes access upstream-owned data.

| Mode   | Ownership Moves | Mutability | Copy | Meaning                     |
| ------ | --------------- | ---------- | ---- | --------------------------- |
| borrow | no              | yes        | no   | temporary mutable access    |
| ref    | no              | no         | no   | shared immutable access     |
| take   | yes             | yes        | no   | ownership transfer          |
| gives  | yes             | no         | yes  | ownership transfer via copy |

---

### 2.3 Ownership Law (Authoritative)

- `borrow` and `ref` **do not move ownership**
- `take` and `gives` **move ownership**
- Only one ownership-moving edge is allowed per value
- Ownership always flows **upstream → downstream**

---

## 3. Types

### 3.1 Built-in Types

```
@int @float @str @bool
@list<T>
@mut list<T>
@dict<K, V>
@table<Row>
@any
```

### 3.2 Traits

```
trait async
trait sync
trait serialize
trait stream
trait copy
```

Traits may participate in type matching.

---

## 4. Variables

```
let x = 10      # immutable
var y = 20      # mutable
```

Variables are **bindings**, not storage.
Storage exists only inside nodes.

---

## 5. Functions (Surface Syntax)

Functions are **syntactic sugar for nodes**.

### 5.1 Basic Function

```
fn add(a, b):
    return a + b
```

Lowered form:

- inputs: borrow
- output: own

---

### 5.2 Typed Function

```
fn add(@int[a, b]) -> @int:
    return result : a + b
```

Lowered to:

- `a`: borrow<int>
- `b`: borrow<int>
- `result`: own<int>

---

### 5.3 Ownership Annotations

```
fn consume(data: take @Image):
    ...

fn view(data: ref @Image):
    ...
```

---

### 5.4 Async Functions

```
async fn fetch():
    return await http.get(url)
```

- `async` affects scheduling only
- Ownership rules remain unchanged

---

## 6. Nodes (Authoritative Form)

### 6.1 Node Definition

```
node add {
    re_in:
        a: borrow @int
        b: borrow @int

    re_out:
        result: own @int
}
```

Nodes define **ownership explicitly**.

---

### 6.2 Node + Function Pair

```
node add { ... }

fn add(@int[a, b]) -> @int:
    return result : a + b
```

Function body provides execution logic.
Node defines contract.

---

## 7. Main Entry (`mn`)

`mn` defines the **application graph**.

### 7.1 Sequential Style

```
mn:
    print(add(2, 3))
```

Lowered to a graph automatically.

---

### 7.2 Explicit Graph Style

```
mn: [
    input(@int(2)) : add: a
    input(@int(3)) : add: b
    print: add: result
]
```

This is the **canonical execution form**.

---

## 8. Control Flow

Control flow compiles to **graph structures**.

### 8.1 If

```
if x > 0:
    print("positive")
elif x < 0:
    print("negative")
else:
    print("zero")
```

Lowered to:

- condition node
- branch nodes
- merge node

---

### 8.2 For

```
for item in items:
    process(item)
```

- iterator node
- body subgraph
- ownership enforced per iteration

---

### 8.3 While

```
while running:
    step()
```

Same lowering as `for` with condition gate.

---

## 9. Mandatory Consumption Rule

Every `own<T>` must:

- be `take`n
- be `gives`n
- be `borrow`ed or `ref`erenced

Unused ownership is a **compile-time error**.

---

## 10. Ownership Validity Examples

### Valid

```
Producer --borrow--> A
Producer --ref--> B
```

### Valid

```
Producer --take--> A
```

### Invalid

```
Producer --take--> A
Producer --borrow--> B   # error
```

---

## 11. Imports and Foreign Code

```
@imp std.http
@imp python{numpy}
```

```
@python { import numpy as np }
@rust { fn compute() -> i32 { 42 } }
```

Foreign code is wrapped as nodes.

---

## 12. Execution Model

1. Build ownership-checked DAG
2. Topological scheduling
3. Ownership locks enforced
4. Parallel execution when allowed

---

## 13. CLI

```
gul run file.mn
gul build file.mn
gul check file.mn
gul fmt file.mn
gul tui
```

---

## 14. Summary

GUL is:

- Data-flow first
- Ownership safe
- Deterministic
- Parallel-ready
- Multi-syntax

The **node graph is the truth**.
Everything else is syntax.

---

# Appendix A: Formal Grammar (EBNF)

```
Program        ::= Import* (Node | Function | Statement)* Main

Import         ::= '@imp' Identifier ('.' Identifier)*

Main           ::= 'mn' ':' (Block | '[' GraphItems ']')

Block          ::= Statement+

Statement      ::= VarDecl
                 | Assignment
                 | FunctionCall
                 | ControlFlow

VarDecl        ::= ('let' | 'var') Identifier '=' Expr

Function       ::= ('async')? 'fn' Identifier '(' Params? ')' ReturnType? ':' Block

Params         ::= Param (',' Param)*
Param          ::= Identifier | Ownership Type Identifier

ReturnType     ::= '->' Type

Node           ::= 'node' Identifier '{' NodeIO '}'
NodeIO         ::= ReIn ReOut
ReIn           ::= 're_in' ':' Port+
ReOut          ::= 're_out' ':' Port+

Port           ::= Identifier ':' Ownership Type

Ownership      ::= 'own' | 'borrow' | 'ref' | 'take' | 'gives'

Type           ::= '@' Identifier | '@' Identifier '<' Type '>'

Expr           ::= Literal
                 | Identifier
                 | FunctionCall
                 | Expr Operator Expr

ControlFlow    ::= If | For | While

If             ::= 'if' Expr ':' Block ('elif' Expr ':' Block)* ('else' ':' Block)?
For            ::= 'for' Identifier 'in' Identifier ':' Block
While          ::= 'while' Expr ':' Block

GraphItems     ::= GraphItem+
GraphItem      ::= 'input' '(' Expr ')' ':' Identifier ':' Identifier
                 | Identifier ':' Identifier ':' Identifier
```

---

# Appendix B: Compiler IR Specification

## B.1 Core IR Types

```
IRNode {
  id: NodeId
  inputs: [IRPort]
  outputs: [IRPort]
  traits: [Trait]
}

IRPort {
  name: String
  type: Type
  ownership: Ownership
}

IREdge {
  from: IRPort
  to: IRPort
  mode: Ownership
}
```

## B.2 IR Graph

```
IRGraph {
  nodes: [IRNode]
  edges: [IREdge]
}
```

Properties:

- DAG unless loop explicitly declared
- Exactly one owner per value
- Ownership moves only via `take` or `gives`

---

# Appendix C: Ownership Inference Rules

Ownership inference is applied **only when not explicitly annotated**.

## C.1 Default Rules

| Context          | Inferred Ownership |
| ---------------- | ------------------ |
| Primitive input  | gives              |
| Struct input     | borrow             |
| Large collection | ref                |
| Function return  | own                |

---

## C.2 Call-Site Inference

When calling `fn f(x)`:

1. If `f` requires `take<T>` → caller must own `T`
2. If `f` requires `borrow<T>` → caller must still own `T`
3. If `f` requires `ref<T>` → caller ownership unchanged

---

## C.3 Conflict Resolution

If multiple ownership modes are possible:

Priority:

1. Explicit annotation
2. Required input satisfaction
3. Minimal ownership movement
4. Shortest graph distance

Ambiguity results in a compile-time error.

---

## C.4 Invalid Inference Cases

- Inferring `take` from `borrow`
- Inferring `gives` for non-copy types
- Inferring ownership movement twice

These are **hard errors**.

---

# Appendix D: Loop and Recursion Ownership Formalization

## D.1 Loops (for / while)

Loops are represented as **subgraphs with iteration boundaries**.

Rules:

1. Ownership **cannot accumulate across iterations**
2. Values entering a loop must be:

   - `borrow`
   - `ref`
   - or explicitly `take`n and returned each iteration

3. Any `own<T>` created inside a loop must be:

   - consumed within the same iteration, or
   - yielded explicitly as loop output

Invalid:

- Creating `own<T>` per iteration without consumption

Valid pattern:

```
for item in items:
    tmp = process(item)
    emit(tmp)
```

---

## D.2 Recursion

Recursive functions are treated as **implicit loops**.

Rules:

1. Recursive calls may not `take` ownership unless it is returned
2. Recursive ownership must be **structurally decreasing** or returned
3. `borrow` and `ref` are always allowed

Illegal:

- Recursive `take` without return

Legal:

- Tail recursion returning ownership

---

# Appendix E: Error Codes and Diagnostics

## E.1 Ownership Errors

| Code | Meaning                         |
| ---- | ------------------------------- |
| E001 | Unused own<T> value             |
| E002 | Multiple ownership-moving edges |
| E003 | Borrow after ownership moved    |
| E004 | Invalid take from non-owner     |
| E005 | Gives on non-copy type          |

---

## E.2 Type Errors

| Code | Meaning                   |
| ---- | ------------------------- |
| E101 | Type mismatch             |
| E102 | Trait not satisfied       |
| E103 | Ambiguous type resolution |

---

## E.3 Graph Errors

| Code | Meaning                        |
| ---- | ------------------------------ |
| E201 | Cycle without loop declaration |
| E202 | Unreachable node               |
| E203 | Dangling graph output          |

Diagnostics must include:

- node name
- port name
- ownership mode
- suggested fix

---

# Appendix F: Standard Library Node Contracts

## F.1 IO

```
node print {
  re_in:
    value: ref @any
}
```

```
node input {
  re_out:
    value: gives @any
}
```

---

## F.2 Math

```
node add {
  re_in:
    a: borrow @int
    b: borrow @int
  re_out:
    result: own @int
}
```

```
node multiply { ... }
```

---

## F.3 Collections

```
node list_push {
  re_in:
    list: borrow mut @list<T>
    value: borrow T
}
```

---

## F.4 Async / IO

```
node http_get {
  re_in:
    url: ref @str
  re_out:
    response: own @bytes
  traits: async
}
```

---

# Appendix G: Reference Execution Engine (VM) Specification

## G.1 VM Overview

The GUL VM executes a **verified IRGraph**.

Pipeline:

1. Ownership verification
2. Topological scheduling
3. Lock acquisition
4. Node execution
5. Ownership transfer or release

---

## G.2 Runtime State

```
VMState {
  values: Map<ValueId, Data>
  owners: Map<ValueId, NodeId>
  locks: Map<ValueId, LockState>
}
```

---

## G.3 Execution Rules

1. A node is runnable when all required inputs are ready
2. `borrow` / `ref` acquire shared locks
3. `take` / `gives` acquire exclusive locks
4. Locks released after node completion

---

## G.4 Parallelism

Nodes may run in parallel if:

- they share no exclusive ownership
- they do not mutate shared data

---

## G.5 Determinism Guarantee

Given the same inputs:

- graph construction is deterministic
- execution order is deterministic
- results are deterministic

---

# End of Specification
