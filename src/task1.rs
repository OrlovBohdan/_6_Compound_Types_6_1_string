#[test]

/*
// Fix error without adding new line
fn main() {
    let s: str = "hello, world";

    println!("Success!");
}
*/

// Fix error without adding new line
fn main() {
    let _s: &str = "hello, world";

    println!("Success!");
}


/*
Ошибка в коде связана с тем, что тип данных str используется неправильно.
В Rust строковые литералы имеют тип &str (ссылка на строку), а не просто str.
Чтобы исправить ошибку, нужно изменить тип переменной s на &str
*/
