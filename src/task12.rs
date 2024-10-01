#[test]

/*

fn main() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".__ {
        println!("{}", c)
    }
}
*/


fn main() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

/*
chars(): Этот метод возвращает итератор по символам строки.
Он правильно обрабатывает символы UTF-8, что особенно важно для таких строк, как "你好，世界", содержащих китайские иероглифы.
*/