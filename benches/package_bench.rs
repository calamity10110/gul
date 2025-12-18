// Performance benchmarks for GUL
#![feature(test)]
extern crate test;

use gul::platform::package_support::PackageManager;
use test::Bencher;

#[bench]
fn bench_package_manager_creation(b: &mut Bencher) {
    b.iter(|| PackageManager::new());
}

#[bench]
fn bench_package_lookup(b: &mut Bencher) {
    let pm = PackageManager::new();
    b.iter(|| pm.has_package("actix-web"));
}

#[bench]
fn bench_package_retrieval(b: &mut Bencher) {
    let pm = PackageManager::new();
    b.iter(|| pm.get_package("actix-web"));
}

#[bench]
fn bench_list_by_language(b: &mut Bencher) {
    let pm = PackageManager::new();
    b.iter(|| pm.list_packages_by_language("rust"));
}
