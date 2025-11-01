fn main() {
    listing1();
    listing2();
    // listing3();
    listing4();
    listing5();
    listing6();
}

fn listing1() {
    let s1 = String::from("hello");
    let ln = calculate_len(&s1);
    println!("The length of '{s1}' is {ln}");
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn listing2() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// fn listing3() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{r1}, {r2}");
// }

fn listing4() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 = {r1}");
    }
    let r2 = &mut s;
    println!("r2 = {r2}");
}

fn listing5() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    let r3 = &mut s;
    println!("{r3}");
}

fn listing6() {
    let ref_to_nothing = dangle();
    println!("{ref_to_nothing}");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn dangle() -> String {
    let s = String::from("hello");
    s
}
