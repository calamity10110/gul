GUL Production Optimization Plan v3.0
A comprehensive roadmap to elevate GUL to production-grade with user-approved syntax changes.

1. Syntax Simplification (User Approved ✅)
   Unified Keyword Mapping
   Concept Old Syntax New Syntax (v3.0) Example
   Imports
   imp
   /
   import
   @imp (3 styles) See import styles below
   Constants
   def
   /
   const
   let (immutable) let pi = 3.14
   Mutables
   mut
   /?var
   var
   (mutable) var time = 1
   Functions
   fn
   fn
   fn greet(name):
   Async Fn async fn
   async
   (no fn) async fetch(url):
   Foreign Code
   cs
   /extern @lang{} @python{}, @rust{}, @cpp{}
   Main Entry
   mn
   /
   main()
   mn: mn: followed by block
   Import Syntax (3 Flexible Styles)
   Style 1: Single Line (per import)

@imp std{io, http, json}
@imp python{numpy, pandas}
Style 2: Block (indented list)

@imp:
std{io, http, json},
python{numpy, pandas},
rust{tokio}
Style 3: Parenthesized (inline list)

@imp(std{io, http, json}, python{numpy, pandas})
All three styles are equivalent and can be mixed in the same file.

Complete Example (v3.0 Syntax)

# imports.def

@imp std{io, http, json}
@imp python{numpy, pandas}

# constants

let PI = 3.14159
let API_URL = "https://api.example.com"
let MAX_RETRIES = 3

# mutable state

var retry_count = 0
var cache = {}

# async function (no fn keyword needed)

async fetch_data(url: str) -> dict:
response = await http.get(url)
return response.json()

# sync function

fn process(data: list) -> list:
return [x * 2 for x in data]

# foreign code blocks

@python {
def analyze(data):
import numpy as np
return {"mean": np.mean(data), "std": np.std(data)}
}
@rust {
fn fast_compute(n: u64) -> u64 {
n _ n _ n
}
}
@cpp {
int fibonacci(int n) {
if (n <= 1) return n;
return fibonacci(n-1) + fibonacci(n-2);
}
}

# main entry point

mn:
let data = await fetch_data(API_URL)
let stats = analyze(data)
print(stats) 2. UI Syntax (User Approved ✅)
Two Equivalent Styles
Decorator Style (standalone):

@ui button{text="Click", color="blue"}
@ui slider{min=0, max=100, value=50}
@ui chart{type="bar", data=values}
@ui table{headers=["Name", "Age"], rows=data}
Function Style (inline):

ui.button(text="Click", color="blue")
ui.slider(min=0, max=100, value=50)
ui.chart(type="bar", data=values)
ui.table(headers=["Name", "Age"], rows=data)
Migration from ^&^[]
Old Syntax New Syntax
^&^[button{text="Click"}] @ui button{text="Click"}
^&^[slider{min=0, max=100}] @ui slider{min=0, max=100}
ui.print(table[...]) ui.table(...) 3. Consistent Patterns (User Approved ✅)
Declaration Pattern

# Always: keyword name: type = value OR keyword name = value

let name: str = "Alice"
let age = 25 # Type inferred
var counter: int = 0
var items = [] # Type inferred
Function Pattern

# Sync: fn name(args) [-> type]:

fn greet(name: str) -> str:
return "Hello, " + name

# Async: async name(args) [-> type]: (no fn keyword)

async fetch(url: str) -> dict:
return await http.get(url)
Block Pattern

# Always: : followed by indented block

if condition:
do_something()
for item in items:
process(item)
while active:
loop_body()
Comment Pattern

# Single line comment

#[
Multi-line comment
spanning multiple lines
]#
Annotation Pattern

# Unified @annotation prefix

@global var app_state = {} # Global mutable
@static let cache = {} # Static immutable
@local var temp = 0 # Explicitly local

# Type annotations in signature

age: int = 25
name: str = "Alice"

# Ownership as keyword prefix

ref data: DataType # Borrowed reference
own buffer: Buffer # Owned (moved)
copy items: list # Copied 4. Auto Build Visual Program Structure
4.1 Visualizer Module
[NEW]
visualizer.rs
//! Program Structure Visualizer
//! Generates diagrams and visual representations of GUL code
use crate::ast::{Program, Statement, Expression};
use std::path::Path;
pub enum VisualizationType {
Mermaid, // Mermaid.js diagrams
Ascii, // ASCII art trees
Flowchart, // Control flow diagrams
Dependencies, // Module dependency graph
CallGraph, // Function call relationships
Ast, // Abstract Syntax Tree
}
pub enum OutputFormat {
Text, // Raw text output
Markdown, // Markdown with embedded diagrams
Html, // Standalone HTML page
Svg, // SVG image
Png, // PNG image (requires external tool)
}
pub struct Visualizer {
program: Program,
viz_type: VisualizationType,
output_format: OutputFormat,
}
impl Visualizer {
pub fn new(program: Program) -> Self { ... }
/// Generate module dependency graph
/// Shows which modules import which other modules
pub fn generate_dependency_graph(&self) -> String {
// Output:
// `mermaid
        // graph TD
        //     main --> std.io
        //     main --> std.http
        //     main --> utils
        //     utils --> std.json
        // `
}
/// Generate function call graph
/// Shows which functions call which other functions
pub fn generate_call_graph(&self) -> String {
// Output:
// `mermaid
        // graph LR
        //     main --> fetch_data
        //     main --> process
        //     fetch_data --> http.get
        //     process --> transform
        // `
}
/// Generate control flow diagram for a specific function
pub fn generate_flowchart(&self, function_name: &str) -> String {
// Output:
// `mermaid
        // flowchart TD
        //     Start --> A[Initialize]
        //     A --> B{condition?}
        //     B -->|Yes| C[Process]
        //     B -->|No| D[Skip]
        //     C --> End
        //     D --> End
        // `
}
/// Generate ASCII tree of file structure
pub fn generate_file_tree(&self, path: &Path) -> String {
// Output:
// project/
// ├── main.mn
// ├── imports.def
// ├── functions.fnc
// ├── utils/
// │ ├── helpers.mn
// │ └── config.def
// └── tests/
// └── test_main.mn
}
/// Generate AST visualization
pub fn generate_ast_tree(&self) -> String {
// Output:
// Program
// ├── Import: std.io
// ├── Import: std.http
// ├── Definition: PI = 3.14
// ├── Function: fetch_data
// │ ├── Param: url: str
// │ └── Body
// │ ├── Await: http.get(url)
// │ └── Return: response.json()
// └── Main
// ├── Call: fetch_data
// └── Call: print
}
/// Generate struct/class relationship diagram
pub fn generate_class_diagram(&self) -> String {
// Output:
// `mermaid
        // classDiagram
        //     class User {
        //         +name: str
        //         +age: int
        //         +greet() str
        //     }
        //     class Admin {
        //         +permissions: list
        //     }
        //     Admin --|> User
        // `
}
}
4.2 CLI Integration
[MODIFY]
main.rs
Add Viz subcommand:

/// Visualize program structure
Viz {
/// Source file or directory
path: PathBuf,
/// Visualization type: deps, calls, flow, tree, ast, class #[arg(short, long, default_value = "deps")]
viz_type: String,
/// Output format: text, md, html, svg #[arg(short, long, default_value = "text")]
output: String,
/// Function name for flowchart #[arg(short, long)]
function: Option<String>,
/// Output file (stdout if not specified) #[arg(short = 'o', long)]
file: Option<PathBuf>,
}
4.3 Usage Examples

# Module dependency graph (Mermaid)

gul viz --type=deps main.mn

# Function call graph

gul viz --type=calls main.mn --output=md > calls.md

# Control flow for specific function

gul viz --type=flow --function=process main.mn

# File structure tree

gul viz --type=tree ./project/

# Full AST tree

gul viz --type=ast main.mn

# Class/struct relationships

gul viz --type=class models.mn --output=svg -o classes.svg 5. Auto Format/Lint/Arrange
5.1 Enhanced Formatter
[MODIFY]
formatter.rs
pub struct Formatter {
config: FormatConfig,
}
pub struct FormatConfig {
indent_size: usize, // Default: 4
max_line_length: usize, // Default: 100
trailing_comma: bool, // Default: true
sort_imports: bool, // Default: true
group_imports: bool, // Default: true
blank_lines_between: usize, // Default: 1
align_assignments: bool, // Default: false
}
impl Formatter {
/// Sort imports alphabetically and group by type
pub fn sort_imports(&self, code: &str) -> String {
// Before:
// @imp random
// @imp std{http}
// @imp python{numpy}
// @imp std{io}
// After:
// @imp std{http, io}
// @imp python{numpy}
// @imp random
}
/// Arrange code blocks in standard order
pub fn arrange_blocks(&self, code: &str) -> String {
// Order:
// 1. Imports (@imp)
// 2. Constants (let)
// 3. Mutable globals (var with @global)
// 4. Type definitions (struct)
// 5. Functions (fn)
// 6. Async functions (async fn)
// 7. Foreign blocks (@python, @rust, etc.)
// 8. Main entry (mn:)
}
/// Normalize whitespace around operators
pub fn normalize_whitespace(&self, code: &str) -> String {
// Before: x=1+2*3
// After: x = 1 + 2 * 3
// Before: fn foo(x:int,y:str)->bool:
// After: fn foo(x: int, y: str) -> bool:
}
/// Smart line wrapping for long lines
pub fn wrap_lines(&self, code: &str) -> String {
// Before:
// let result = some_long_function(argument1, argument2, argument3, argument4, argument5)
// After:
// let result = some_long_function(
// argument1,
// argument2,
// argument3,
// argument4,
// argument5,
// )
}
/// Add/remove trailing commas consistently
pub fn normalize_trailing_commas(&self, code: &str) -> String
/// Full format pass
pub fn format(&self, code: &str) -> String {
let code = self.sort_imports(code);
let code = self.arrange_blocks(&code);
let code = self.normalize_whitespace(&code);
let code = self.wrap_lines(&code);
let code = self.normalize_trailing_commas(&code);
code
}
}
5.2 Enhanced Linter
[MODIFY]
linter.rs
pub struct Linter {
rules: Vec<Box<dyn LintRule>>,
config: LintConfig,
}
pub struct LintConfig {
severity_threshold: Severity, // Ignore below this
fix_mode: bool, // Auto-fix issues
custom_rules: Vec<String>, // Additional rule files
}
pub enum Severity {
Hint, // Suggestion
Info, // Information
Warning, // Should fix
Error, // Must fix
}
pub struct LintIssue {
rule: String,
severity: Severity,
message: String,
line: usize,
column: usize,
suggestion: Option<String>, // Auto-fix suggestion
}
// Built-in Rules
pub struct NamingConventionRule; // snake_case, PascalCase
pub struct UnusedImportRule; // Warn on unused imports
pub struct UnusedVariableRule; // Warn on unused variables
pub struct MutableConstantRule; // Error if mutating `let`
pub struct OwnershipViolationRule; // Borrow checker warnings
pub struct UnitConsistencyRule; // `m + kg` is error
pub struct ComplexityRule; // Cyclomatic complexity > 10
pub struct DeprecatedSyntaxRule; // Old syntax detection
pub struct AIFriendlinessRule; // Check AI-friendly patterns
impl Linter {
pub fn lint(&self, code: &str) -> Vec<LintIssue> {
let mut issues = vec![];
for rule in &self.rules {
issues.extend(rule.check(code));
}
issues.sort_by_key(|i| (i.line, i.column));
issues
}
pub fn fix(&self, code: &str) -> String {
let mut fixed = code.to_string();
for issue in self.lint(code) {
if let Some(suggestion) = issue.suggestion {
fixed = apply_fix(&fixed, &issue, &suggestion);
}
}
fixed
}
}
5.3 Code Arranger
[NEW]
arranger.rs
//! Code Arranger - Split/Merge GUL files into block structure
pub struct ArrangedFiles {
pub imports: String, // imports.def
pub definitions: String, // definitions.def
pub functions: String, // functions.fnc
pub async_fns: String, // async.fnc
pub foreign: String, // foreign.cs
pub main: String, // main.mn
}
pub struct CodeArranger;
impl CodeArranger {
/// Split single .mn file into organized block files
pub fn split(source: &str) -> ArrangedFiles {
// Parse and categorize each statement
// Return separate files for each category
}
/// Merge block files back into single .mn
pub fn merge(files: ArrangedFiles) -> String {
// Combine in standard order:
// imports -> definitions -> functions -> async -> foreign -> main
}
/// Write arranged files to directory
pub fn write_to_dir(files: &ArrangedFiles, dir: &Path) -> Result<()>
/// Read arranged files from directory
pub fn read_from_dir(dir: &Path) -> Result<ArrangedFiles>
}
5.4 File Watcher
[NEW]
watch.rs
//! File Watcher - Hot reload on file changes
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;
pub struct FileWatcher {
paths: Vec<PathBuf>,
debounce: Duration,
}
pub enum WatchEvent {
Created(PathBuf),
Modified(PathBuf),
Deleted(PathBuf),
}
pub struct WatchConfig {
pub debounce_ms: u64, // Default: 100
pub recursive: bool, // Default: true
pub ignore_patterns: Vec<String>, // Default: [".git", "target"]
}
impl FileWatcher {
pub fn new(paths: Vec<PathBuf>, config: WatchConfig) -> Self
/// Start watching (blocking)
pub fn watch<F>(&self, callback: F) -> Result<()>
where
F: Fn(WatchEvent) -> Result<()>
/// Start watching (non-blocking, returns channel)
pub fn watch_async(&self) -> mpsc::Receiver<WatchEvent>
}
// CLI usage in main.rs:
// gul watch main.mn
// -> Watches file, on change:
// 1. Format
// 2. Lint
// 3. Check (type check)
// 4. Run tests
// 5. Report results 6. Auto Testing Infrastructure
6.1 Native Test Framework
[NEW]
test_runner.rs
//! GUL Native Test Runner
pub struct TestRunner {
test_files: Vec<PathBuf>,
config: TestConfig,
}
pub struct TestConfig {
pub parallel: bool, // Run tests in parallel
pub timeout: Duration, // Per-test timeout
pub coverage: bool, // Collect coverage
pub filter: Option<String>, // Filter by test name
pub verbose: bool, // Detailed output
}
pub struct TestCase {
pub name: String,
pub file: PathBuf,
pub line: usize,
pub is_async: bool,
pub tags: Vec<String>, // @slow, @integration, etc.
}
pub struct TestResult {
pub case: TestCase,
pub status: TestStatus,
pub duration: Duration,
pub output: String,
pub error: Option<String>,
}
pub enum TestStatus {
Passed,
Failed,
Skipped,
Timeout,
}
pub struct TestSummary {
pub total: usize,
pub passed: usize,
pub failed: usize,
pub skipped: usize,
pub duration: Duration,
pub coverage: Option<CoverageReport>,
}
impl TestRunner {
/// Discover all test functions in path
pub fn discover(&self, path: &Path) -> Vec<TestCase> {
// Find all functions with @test annotation
}
/// Run all discovered tests
pub fn run(&self) -> TestSummary
/// Run specific test
pub fn run_one(&self, test: &TestCase) -> TestResult
/// Generate coverage report
pub fn coverage(&self) -> CoverageReport
}
6.2 Test Syntax in GUL

# tests/test_math.mn

@imp std.test

# Basic test

@test
fn test_addition():
assert 1 + 1 == 2
assert 5 - 3 == 2

# Test with description

@test("should calculate area correctly")
fn test_area():
let area = 5 \* 10
assert area == 50

# Async test

@test
@async
async test_http_fetch():
let response = await http.get("https://httpbin.org/get")
assert response.status == 200
assert response.body != ""

# Test with expected error

@test
@expect_error("DivisionByZero")
fn test_division_by_zero():
let x = 1 / 0

# Parameterized test

@test
@params([
(1, 1, 2),
(2, 2, 4),
(0, 0, 0),
(-1, 1, 0),
])
fn test_add_params(a: int, b: int, expected: int):
assert a + b == expected

# Property-based test

@test
@property(iterations=100)
fn test_sort_preserves_length(items: list[int]):
let sorted = sort(items)
assert len(sorted) == len(items)

# Slow test (excluded by default)

@test
@slow
fn test_large_computation(): # Takes > 1 second
let result = heavy_calculation()
assert result > 0

# Skip test

@test
@skip("Not implemented yet")
fn test_future_feature():
pass

# Setup and teardown

@before_each
fn setup():
var test_db = create_test_database()
@after_each
fn teardown():
test_db.cleanup()
@before_all
fn global_setup():
print("Starting test suite")
@after_all
fn global_teardown():
print("Finished test suite")
6.3 CLI Commands

# Run all tests

gul test

# Run specific file

gul test tests/test_math.mn

# Run with filter

gul test --filter="test_add\*"

# Run with coverage

gul test --coverage

# Run including slow tests

gul test --include-slow

# Run in verbose mode

gul test --verbose

# Watch mode (re-run on changes)

gul test --watch 7. Auto Documentation System
7.1 Doc Generator
[NEW]
docgen.rs
//! Documentation Generator
pub struct DocGenerator {
source_dirs: Vec<PathBuf>,
output_dir: PathBuf,
config: DocConfig,
}
pub struct DocConfig {
pub format: DocFormat,
pub include_private: bool,
pub include_source: bool,
pub include_examples: bool,
pub generate_search_index: bool,
}
pub enum DocFormat {
Markdown,
Html,
Json,
}
pub struct DocItem {
pub kind: DocItemKind,
pub name: String,
pub signature: String,
pub description: String,
pub params: Vec<ParamDoc>,
pub returns: Option<ReturnDoc>,
pub examples: Vec<String>,
pub see_also: Vec<String>,
pub since: Option<String>,
pub deprecated: Option<String>,
}
impl DocGenerator {
/// Extract documentation from source files
pub fn extract(&self) -> Vec<DocItem>
/// Generate markdown documentation
pub fn generate_markdown(&self) -> Result<()>
/// Generate HTML documentation site
pub fn generate_html(&self) -> Result<()>
/// Generate JSON API for search
pub fn generate_json(&self) -> Result<()>
/// Generate module index page
pub fn generate_index(&self) -> String
/// Generate search index
pub fn generate_search_index(&self) -> String
}
7.2 Doc Comment Syntax #[
Fetches user data from the API.
Performs an async HTTP GET request to retrieve user information
based on the provided user ID.
@param user_id - The unique identifier for the user (positive integer)
@param options - Optional request configuration
- timeout: Request timeout in seconds (default: 30)
- retry: Number of retries on failure (default: 3)
@returns User object containing name, email, and profile data,
or None if user is not found
@throws NetworkError - If connection fails
@throws AuthError - If authentication is invalid
@example
# Fetch a single user
let user = await fetch_user(123)
if user:
print(user.name)
@example
# Fetch with options
let user = await fetch_user(
456,
options={timeout=10, retry=1}
)
@see User, UserProfile, fetch_users
@since 0.14.0
@deprecated Use fetch_user_v2 instead (removed in 1.0)
]#
async fn fetch_user(user_id: int, options: dict = {}) -> User?:
... #[
User data model.
Represents a user account in the system with basic profile information.
@field name - Display name of the user
@field email - Primary email address
@field age - Age in years (optional)
@field created_at - Account creation timestamp
]#
struct User:
name: str
email: str
age: int?
created_at: datetime
7.3 Generated Documentation

# Module: users

## Functions

### `fetch_user`

```gul
async fn fetch_user(user_id: int, options: dict = {}) -> User?
```

Fetches user data from the API.

Parameters:

Name Type Description
user_id
int
The unique identifier for the user
options dict Optional request configuration
Returns: User? - User object or None if not found

Raises:

NetworkError - If connection fails
AuthError - If authentication is invalid
Example:

let user = await fetch_user(123)
if user:
print(user.name)
Structs
User
User data model.

Fields:

Name Type Description
name
str
Display name of the user
email
str
Primary email address
age
int? Age in years (optional)
created_at datetime Account creation timestamp

### 7.4 CLI Commands

```bash
# Generate markdown docs
gul doc --format=md --output=docs/api/
# Generate HTML site
gul doc --format=html --output=docs/site/
# Generate for specific module
gul doc src/users.mn --output=docs/users.md
# Generate and serve locally
gul doc --serve --port=8080
# Check doc coverage
gul doc --check  # Error if public items undocumented
8. Production Infrastructure
8.1 Enhanced Error Handling
[MODIFY]
interpreter.rs
pub struct GulError {
    pub kind: ErrorKind,
    pub message: String,
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub source_line: String,
    pub suggestion: Option<String>,
    pub stack_trace: Vec<StackFrame>,
}
pub struct StackFrame {
    pub function: String,
    pub file: PathBuf,
    pub line: usize,
}
impl Display for GulError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Output:
        // error[E0123]: undefined variable 'usre'
        //  --> src/main.mn:15:10
        //    |
        // 15 |     print(usre.name)
        //    |           ^^^^ undefined variable
        //    |
        //    = help: did you mean 'user'?
        //    |
        // 14 |     let user = fetch_user(123)
        //    |         ---- defined here
        //
        // Stack trace:
        //   at main() in src/main.mn:15
        //   at process() in src/utils.mn:42
    }
}
8.2 Structured Logging
[NEW]
logging.rs
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
pub enum LogFormat {
    Text,      // Human readable
    Json,      // Machine parseable
    Compact,   // Minimal output
}
pub struct Logger {
    level: LogLevel,
    format: LogFormat,
    outputs: Vec<Box<dyn LogOutput>>,
}
pub trait LogOutput: Send + Sync {
    fn write(&self, record: &LogRecord);
}
pub struct LogRecord {
    pub timestamp: DateTime,
    pub level: LogLevel,
    pub target: String,      // Module path
    pub message: String,
    pub fields: HashMap<String, Value>,
}
impl Logger {
    pub fn trace(&self, msg: &str) { ... }
    pub fn debug(&self, msg: &str) { ... }
    pub fn info(&self, msg: &str) { ... }
    pub fn warn(&self, msg: &str) { ... }
    pub fn error(&self, msg: &str) { ... }
    pub fn with_field(&self, key: &str, value: impl Into<Value>) -> Logger
}
// Usage in GUL:
// @imp std.log
//
// log.info("User logged in", user_id=123)
// log.error("Request failed", error=err, retry=3)
8.3 Runtime Metrics
[NEW]
metrics.rs
pub struct Metrics {
    counters: HashMap<String, AtomicU64>,
    gauges: HashMap<String, AtomicI64>,
    histograms: HashMap<String, Histogram>,
}
impl Metrics {
    /// Increment a counter
    pub fn count(&self, name: &str, value: u64)
    /// Set a gauge value
    pub fn gauge(&self, name: &str, value: i64)
    /// Record a histogram value (for timing, sizes, etc.)
    pub fn record(&self, name: &str, value: f64)
    /// Time a block of code
    pub fn time<F, R>(&self, name: &str, f: F) -> R
    where
        F: FnOnce() -> R
    /// Export metrics as JSON
    pub fn export_json(&self) -> String
    /// Export metrics as Prometheus format
    pub fn export_prometheus(&self) -> String
}
// Usage in GUL:
// @imp std.metrics
//
// metrics.count("requests_total", 1)
// metrics.gauge("active_connections", connections.len())
//
// @timed("process_duration")
// fn process(data):
//     ...
8.4 Configuration Management
[NEW]
config.rs
pub struct Config {
    sources: Vec<ConfigSource>,
    values: HashMap<String, Value>,
}
pub enum ConfigSource {
    File(PathBuf),           // config.toml, config.json
    Env,                     // Environment variables
    Args,                    // Command line arguments
    Remote(String),          // URL to fetch config
}
impl Config {
    /// Load from multiple sources with priority
    pub fn load(sources: Vec<ConfigSource>) -> Result<Self>
    /// Get typed value
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T>
    /// Get with default
    pub fn get_or<T>(&self, key: &str, default: T) -> T
    /// Watch for changes (hot reload)
    pub fn watch(&self, callback: impl Fn(&Config))
}
// Usage in GUL:
// @imp std.config
//
// let config = Config.load([
//     ConfigSource.File("config.toml"),
//     ConfigSource.Env,
// ])
//
// let db_url = config.get("database.url")
// let port = config.get_or("server.port", 8080)
Implementation Priority
Phase	Components	Effort	Status
Phase 1	Syntax updates (lexer, parser)	2 weeks	🔴 High Priority
Phase 2	Formatter + Linter enhancement	1 week	🔴 High Priority
Phase 3	Visualizer module	1 week	🟡 Medium
Phase 4	Watch + Arranger	4 days	🟡 Medium
Phase 5	Test runner	1 week	🟡 Medium
Phase 6	Doc generator	1 week	🟡 Medium
Phase 7	Logging + Metrics	5 days	🟢 Low
Phase 8	Error handling upgrade	3 days	🟢 Low
Verification Plan
Syntax Tests
# Update lexer tests
cargo test lexer -- --nocapture
# Update parser tests
cargo test parser -- --nocapture
# Verify all examples compile with new syntax
for f in examples/*.mn; do gul check "$f"; done
Tool Tests
# Format and verify
gul fmt examples/ && gul check examples/
# Lint clean check
gul lint examples/ --strict
# Visualization smoke test
gul viz --type=deps examples/hello_world.mn
Documentation Tests
# Generate docs
gul doc src/ --output=docs/api/
# Build MkDocs site
mkdocs build --strict
# Check all links
linkchecker docs/site/
Integration Tests
# Full pipeline test
gul new test-project
cd test-project
gul fmt main.mn
gul lint main.mn
gul check main.mn
gul run main.mn
gul test
gul doc
gul viz --type=deps main.mn
```
