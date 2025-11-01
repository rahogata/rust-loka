fn main() {
    listing1();
    // listing2();
    listing3();
    listing4();
    listing5();
    listing6();
    listing7();
}

fn listing1() {
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{s1}");
}

// fn listing2() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{s1}, world!");
// }

fn listing3() {
    let mut s = String::from("hello");
    println!("s before = {s}");
    s = String::from("ahoy");
    println!("{s}, world!");
}

fn listing4() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
}

fn listing5() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}

fn listing6() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {s1}");
    println!("s3 = {s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn listing7() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
