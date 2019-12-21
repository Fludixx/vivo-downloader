pub fn caesar(input: String, alphabet: &str, shift: i32) -> String {
    let len = alphabet.len();
    let mut out = String::new();
    for c in input.chars() {
        if alphabet.contains(c) {
            out.push(
                alphabet.as_bytes()[((alphabet.find(c).unwrap() + shift as usize) % len)] as char,
            );
        } else {
            out.push(c);
        }
    }
    out
}

pub fn rot47(input: String) -> String {
    caesar(input, "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 47)
}
