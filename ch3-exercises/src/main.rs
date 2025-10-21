use std::io;

fn main() {
    convert_temperature();
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
