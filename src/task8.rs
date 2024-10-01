#[test]

/*

// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = s;

    println!("Success!");
}
*/


/*// Use two approaches to fix the error and without adding a new line
//1
fn main() {
    let s = "hello, world".to_string();
    let _s1: &str = &s;

    println!("Success!");
}*/

// Use two approaches to fix the error and without adding a new line
//2
fn main() {
    let s = "hello, world".to_string();
    let _s1: &str = &s[..];

    println!("Success!");
}

/*
Ошибка возникает, потому что переменная s имеет тип String, а вы пытаетесь присвоить её переменной типа &str.
Чтобы исправить это без добавления новых строк, можно применить два подхода:
Использование ссылки на строку String
Преобразование строки String в строковый срез
*/