#[test]



/* Fill in the blank and fix the errors
fn main() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    // Modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = __;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}*/

fn main() {
    // Изменяем строку, чтобы экранирование работало
    let raw_str = "Escapes don't work here: ? ℝ"; // изменили на обычную строку
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // Если вам нужны кавычки в сырой строке, добавьте пару #.
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // Если вам нужно "# в вашей строке, просто используйте больше # в ограничителе.
    // Вы можете использовать до 65535 #.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Заполняем пропуск с помощью сырой строки
    let long_delimiter = r###"Hello, "##""###; // Используем три # в начале и в конце
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}
/*
В строке let long_delimiter = r###"Hello, "##""###
я использовал три символа # в начале и в конце. Это позволяет включать ## внутри строки.
*/






