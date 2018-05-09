#![feature(test)]
extern crate fastcopy as fcopy;
extern crate tempfile;
extern crate test;

use std::fs;
use std::io::Write;
use test::Bencher;

#[bench]
fn small_file_fastcopy(b: &mut Bencher) {
    let mut source_file = tempfile::NamedTempFile::new().unwrap();
    write!(source_file, "Hello World!").unwrap();
    let source_file = source_file.into_temp_path();
    let target_file = tempfile::NamedTempFile::new().unwrap().into_temp_path();

    b.iter(|| {
        fcopy::copy(&source_file, &target_file).expect("error copying file");
    });
}

#[bench]
fn small_file_std(b: &mut Bencher) {
    let mut source_file = tempfile::NamedTempFile::new().unwrap();
    write!(source_file, "Hello World!").unwrap();
    let source_file = source_file.into_temp_path();
    let target_file = tempfile::NamedTempFile::new().unwrap().into_temp_path();

    b.iter(|| {
        fs::copy(&source_file, &target_file).expect("error copying file");
    });
}

#[bench]
fn mb_10_fastcopy(b: &mut Bencher) {
    let mut source_file = tempfile::NamedTempFile::new().unwrap();
    let content: Vec<u8> = (0..10 * 1024 * 1024).map(|_| b'a').collect();
    source_file.write_all(&content).unwrap();
    let target_file = tempfile::NamedTempFile::new().unwrap();

    b.iter(|| {
        fcopy::copy(&source_file, &target_file).expect("error copying file");
    });
}

#[bench]
fn mb_10_std(b: &mut Bencher) {
    let mut source_file = tempfile::NamedTempFile::new().unwrap();
    let content: Vec<u8> = (0..10 * 1024 * 1024).map(|_| b'a').collect();
    source_file.write_all(&content).unwrap();
    let target_file = tempfile::NamedTempFile::new().unwrap();

    b.iter(|| {
        fs::copy(&source_file, &target_file).expect("error copying file");
    });
}

#[bench]
fn mb_100_fastcopy(b: &mut Bencher) {
    let mut source_file = tempfile::NamedTempFile::new().unwrap();
    let content: Vec<u8> = (0..1024 * 1024 * 100).map(|_| b'a').collect();
    source_file.write_all(&content).unwrap();
    let target_file = tempfile::NamedTempFile::new().unwrap();

    b.iter(|| {
        fcopy::copy(&source_file, &target_file).expect("error copying file");
    });
}

#[bench]
fn mb_100_std(b: &mut Bencher) {
    let mut source_file = tempfile::NamedTempFile::new().unwrap();
    let content: Vec<u8> = (0..1024 * 1024 * 100).map(|_| b'a').collect();
    source_file.write_all(&content).unwrap();
    let target_file = tempfile::NamedTempFile::new().unwrap();

    b.iter(|| {
        fs::copy(&source_file, &target_file).expect("error copying file");
    });
}
