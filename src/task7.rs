#[test]

/*

// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s)
}

fn greetings(s: String) {
    println!("{}", s)
}
*/



/*
// Fix error with at least two solutions
//1
fn main() {
    let s = "hello, world";
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}", s)
}*/

// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s.to_string())
}

fn greetings(s: String) {
    println!("{}", s)
}


/*
Ошибка возникает потому, что в функции greetings параметр ожидает строку типа String,
а вы передаете строковый срез (&str), который имеет другой тип. Нужно либо изменить
тип параметра в функции, либо преобразовать строковый срез в тип String.
*/

