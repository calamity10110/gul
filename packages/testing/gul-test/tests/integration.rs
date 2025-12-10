use gul_test::TestSuite;
use std::path::PathBuf;

#[test]
fn test_gul_packages() {
    let mut suite = TestSuite::new("GUL Integration Tests");

    // Find all .gul files in gul_packages/tests
    let test_dir = PathBuf::from("../../../gul_packages/tests");
    if test_dir.exists() {
        for entry in std::fs::read_dir(test_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("gul") {
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                suite.add_file_test(name, path);
            }
        }
    }

    // Also run the simple_test from root if it exists
    let simple_test = PathBuf::from("../../../simple_test.gul");
    if simple_test.exists() {
        suite.add_file_test("simple_test", simple_test);
    }

    let results = suite.run();
    println!("{}", results.summary());

    assert!(results.is_success(), "Some GUL tests failed");
}
