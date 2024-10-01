#[test]
/*

// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}
*/

fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

/*
Ошибка возникает из-за того, что операция сложения строк в Rust перемещает владение
первой строки (s1) и пытается конкатенировать с помощью ссылки на вторую строку (s2).
Однако строка s1 больше недоступна после этой операции, что приводит к ошибке при попытке
использовать её позже.
*/

