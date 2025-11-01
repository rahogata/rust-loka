fn main() {
    let mut s = String::from("jai hanuman jyana guna sagar");
    let w = first_word(&s);
    println!("{w}");
    s.clear();
    // println!("{w}"); compilation error
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &e) in bytes.iter().enumerate() {
        if e == b' ' {
            return &s[0..i];
        }
    }
    return &s[..]
}