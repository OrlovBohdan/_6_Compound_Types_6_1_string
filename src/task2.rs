#[test]

/*
// Fix the error with at least two solutions
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}
*/

// Fix the error with at least two solutions
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}",s)
}

/*
В данном коде проблема заключается в том, что в функцию greetings передаётся тип Box<str>,
тогда как функция ожидает параметр типа &str
*/