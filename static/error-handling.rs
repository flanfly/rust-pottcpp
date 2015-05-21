fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
}

fn main() {
    let filename = "foobar.txt";
    match find(filename, '.') {
        Some(i) => println!("Filename extension: {}", &filename[i+1..]),
    }
}
