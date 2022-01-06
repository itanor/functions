use std::fmt;

struct User {
    id: u64,
    name: String
}

fn main() {
    print_labeled_measurement(5, 'h');
    println!("{}", five());

    let user = new_user(34, String::from("john"));
    println!("{}", user);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("param value: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn new_user(id: u64, name: String) -> User {
    User{id, name}
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User id: {}, name: {}", self.id, self.name)
    }
}
