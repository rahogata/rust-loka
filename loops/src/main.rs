fn main() {
    listing1();
    listing2();
    listing3();
    listing4();
    listing5();
    listing6();
}

fn listing1() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn listing2() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn listing3() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn listing4() {
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn listing5() {
    let a = [10,20,30,40,50];
    for element in a {
        println!("the value is from for: {element}");
    }
}

fn listing6() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}