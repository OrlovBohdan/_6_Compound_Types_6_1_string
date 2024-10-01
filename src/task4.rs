#[test]

/*
// Fix all errors without adding newline
fn main() {
    let s = String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();

    println!("{}", s);
}
*/


// Fix all errors without adding newline
fn main() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

/*
Метод push для строки типа String принимает только символ (типа char),
но происходит попытка передать строку (типа &str) в вызове s.push(" world").
В выражении s += "!".to_string() используется тип String, но += оператор ожидает тип &str.
*/


