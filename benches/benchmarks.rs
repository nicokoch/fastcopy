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

#[bench]
fn copy_100_small_files_fastcopy(b: &mut Bencher) {
    let source_files: Vec<_> = (0..100).map(|_i| {
        let mut source_file = tempfile::NamedTempFile::new().unwrap();
        write!(source_file, "Hello World!").unwrap();
        source_file.into_temp_path()
    }).collect();
    let target_file = tempfile::NamedTempFile::new().unwrap().into_temp_path();

    b.iter(|| {
        for source_file in &source_files {
            fcopy::copy(&source_file, &target_file).expect("error copying file");
        }
    });
}

#[bench]
fn copy_100_small_files_std(b: &mut Bencher) {
    let source_files: Vec<_> = (0..100).map(|_i| {
        let mut source_file = tempfile::NamedTempFile::new().unwrap();
        write!(source_file, "Hello World!").unwrap();
        source_file.into_temp_path()
    }).collect();
    let target_file = tempfile::NamedTempFile::new().unwrap().into_temp_path();

    b.iter(|| {
        for source_file in &source_files {
            fs::copy(&source_file, &target_file).expect("error copying file");
        }
    });
}

use std::path::PathBuf;

#[bench]
fn copy_100_small_files_across_fs_fastcopy(b: &mut Bencher) {
    let source_files: Vec<_> = (0..100).map(|i| {
        let mut path = PathBuf::new();
        path.push("source_files/");
        path.push(i.to_string());
        let mut source_file = fs::File::create(&path).unwrap();
        write!(source_file, "Hello World!").unwrap();
        path
    }).collect();
    let target_file = "tmpfs/target.txt";
    b.iter(|| {
        for source_file in &source_files {
            fcopy::copy(&source_file, &target_file).expect("error copying file");
        }
    });
}

#[bench]
fn copy_100_small_files_across_fs_std(b: &mut Bencher) {
    let source_files: Vec<_> = (0..100).map(|i| {
        let mut path = PathBuf::new();
        path.push("source_files/");
        path.push(i.to_string());
        let mut source_file = fs::File::create(&path).unwrap();
        write!(source_file, "Hello World!").unwrap();
        path
    }).collect();
    let target_file = "tmpfs/target.txt";
    b.iter(|| {
        for source_file in &source_files {
            fs::copy(&source_file, &target_file).expect("error copying file");
        }
    });
}
