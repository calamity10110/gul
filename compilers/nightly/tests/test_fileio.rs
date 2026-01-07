// Test file I/O
println!("{}", "Testing file I/O...");

// Write a test file
let content = "Hello from GUL compiler test!";
let success = write_file("test_output.txt", content);
println!("{}", "Write successful:", success);

// Read it back
let read_content = read_file("test_output.txt");
println!("{}", "Read content:", read_content);

// Check file exists
let exists = file_exists("test_output.txt");
println!("{}", "File exists:", exists);

println!("{}", "File I/O test complete!");