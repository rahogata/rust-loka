use std::io;

fn main() {
    // convert_temperature();
    // fibonacci();
    chrismas12days();
}

fn convert_temperature() {
    println!("{}", "Enter temperature:");
    let temp: f64 = loop {
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temperature");
        match temp.trim().parse() {
            Ok(n) => break n,
            Err(_) => {
                println!("Enter temperature as number:");
                continue
            },
        };
    };
    loop {
        println!("Enter temperature type (C) or (F):");
        let mut temp_type = String::new();
        io::stdin()
            .read_line(&mut temp_type)
            .expect("Failed to read temperature type");
        let temp_type = temp_type.trim();
        if temp_type == "C" {
            let f = (temp * (9.0 / 5.0)) + 32.0;
            println!("{temp} °C = {f} °F");
            break;
        } else if temp_type == "F" {
            let cel = (temp - 32.0) * (5.0 / 9.0);
            println!("{temp} °F = {cel} °C");
            break;
        } else {
            continue
        }
    }
}

fn fibonacci() {
    let n = loop {
        println!("Enter how many terms of Fibonacci to print:");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
        match n.trim().parse() {
            Ok(n) => break n,
            Err(_) => continue,
        }
    };
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        print!("{a} ");
        let next = a + b;
        a = b;
        b = next;
    }
    println!();
}

fn chrismas12days() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = [
        "a Partridge in a Pear Tree.",
        "two Turtle Doves,",
        "three French Hens,",
        "four Calling Birds,",
        "five Gold Rings,",
        "six Geese a-Laying,",
        "seven Swans a-Swimming,",
        "eight Maids a-Milking,",
        "nine Ladies Dancing,",
        "ten Lords a-Leaping,",
        "eleven Pipers Piping,",
        "twelve Drummers Drumming,",
    ];
    print_start_msg(days[0]);
    println!("{}", gifts[0]);
    for day in 1..12 {
        print_start_msg(days[day]);
        for gift_idx in (1..day+1).rev() {
            println!("{}", gifts[gift_idx]);
        }
        println!("and {}", gifts[0]);
    }
}

fn print_start_msg(d: &str) {
    println!("\nOn the {} day of Christmas my true love sent to me:", d);
}
