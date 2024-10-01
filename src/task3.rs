#[test]

/*
// Fill the blank
fn main() {
    let mut s = __;
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
*/

// Fill the blank
fn main() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

/*
В этом случае строка изменяется с помощью методов push_str и push,
которые добавляют текст к строке. Для того чтобы это работало, переменная
s должна быть изменяемой строкой. В Rust изменяемые строки имеют тип String.
*/