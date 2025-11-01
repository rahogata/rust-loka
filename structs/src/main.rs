struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let u = User {
        active: true,
        username: String::from("rama"),
        email: String::from("rama@example.com"),
        sign_in_count: 2,
    };
    let mut u1 = User {
        active: false,
        username: String::from("seeta"),
        email: String::from("seeta@example.com"),
        sign_in_count: 3,
    };
    u1.email = String::from("janaki@example.com");
    let u2 = build_user(String::from("chowdamma@example.com"), String::from("chowdamma"));
    let u3 = build_user_short(String::from("cheluva@example.com"), String::from("cheluva"));
    let u4 = duplicate_user(u3);
    let u5 = duplicate_user_short(u4);
    println!("u:{0}, u1:{1}, u2:{2}, u3:{3}", u.email, u1.email, u2.email, u5.email);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("b:{0}, o:{1}", black.0, origin.0);
    let Point(x, y, z) = origin;
    println!("x:{x}, y:{y}, z:{z}");
    let sub = AlwaysEqual;
    
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 4,
    }
}

fn build_user_short(email: String, username: String) -> User {
    User {
        active: false,
        email,
        username,
        sign_in_count: 5,
    }
}

fn duplicate_user(u: User) -> User {
    User {
        active: u.active,
        username: u.username,
        email: String::from("kausalya@example.com"),
        sign_in_count: u.sign_in_count,
    }
}

fn duplicate_user_short(u: User) -> User {
    User {
        email: String::from("sumitra@example.com"),
        ..u
    }
}
