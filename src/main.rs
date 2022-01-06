use std::fmt;

struct User {
    id: u64,
    name: String
}

fn main() {
    print_labeled_measurement(5, 'h');
    println!("{}", five_plus_or_zero(22));

    let user = new_user(34, String::from("john"));
    println!("{}", user);

    labeled_loop();
    returning_values_from_loops();
    for_loop();
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("param value: {}{}", value, unit_label);
}

fn five_plus_or_zero(number: u64) -> u64 {
    let result = if number > 10 {5 + number} else {0};
    result
}

fn new_user(id: u64, name: String) -> User {
    User{id, name}
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User id: {}, name: {}", self.id, self.name)
    }
}

fn labeled_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

fn returning_values_from_loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn for_loop() {
    let array = [10, 20, 30, 40, 50];
    for element in array {
        println!("value is: {}", element);
    }
    for element in (1..5).rev() {
        println!("{}", element);
    }
}

