extern crate fastcopy as fcopy;

use std::fs;
use std::io::Write;
fn main() {
    {
        let mut f = fs::File::create("abc.txt").expect("Unable to create file");
        let content: Vec<u8> = (0..100 * 1024 * 1024).map(|_| b'a').collect();
        f.write_all(&content).unwrap();
    }
    fcopy::copy("abc.txt", "cba.txt").expect("error copying file");
}
