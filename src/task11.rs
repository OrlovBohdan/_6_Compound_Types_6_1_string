#[test]

/*

fn main() {
    let s1 = String::from("hi,中国");
    let h = s1[0]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..5]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}
*/


fn main() {
    let s1 = String::from("hi,中国");

    // Чтобы получить первый символ правильно, мы используем метод chars()
    let h = s1.chars().nth(0).unwrap(); // Это даст нам 'h'
    assert_eq!(h, 'h');

    // Чтобы извлечь символ '中', мы также можем использовать chars()
    let h1: String = s1.chars().nth(3).unwrap().to_string(); // Это даст нам "中"
    assert_eq!(h1, "中");

    println!("Success!");
}




/*
Получение первого символа:
Вместо прямого индексирования строки с помощью s1[0], что не работает для символов в кодировке UTF-8,
мы используем s1.chars().nth(0).unwrap(), чтобы безопасно получить первый символ.
Это возвращает значение типа Option, поэтому мы используем unwrap(), чтобы получить значение.

Получение символа '中':
Аналогично первому символу, мы используем s1.chars().nth(3).unwrap(), чтобы получить символ по индексу 3.
Затем мы конвертируем его в String с помощью to_string(), так как assert_eq! сравнивает с &str.
*/



