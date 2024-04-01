use std::io;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

fn main() {
    let secret_number: u32 = thread_rng().gen_range(1..=100);

    println!("Угадайте число");
    println!("--dev-secret-number-is: {secret_number}");

    loop {
        println!("Пожалуйста введите ваше предположение:");
        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Ошибка чтения ввода");

        let guess: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Введите число");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое"),
            Ordering::Greater => println!("Слишком большое"),
            Ordering::Equal => {
                println!("Вы выиграли");
                break;
            }
        }
    }
}
