use std::collections::HashSet;

fn bruteforce_recursive(password: &str, current_attempt: &mut String, charset: &str, max_length: usize) {
    if current_attempt.len() == max_length {
        if &current_attempt == password {
            println!("Пароль найден: {}", current_attempt);
        }
        return;
    }

    for character in charset.chars() {
        current_attempt.push(character);
        bruteforce_recursive(password, current_attempt, charset, max_length);
        current_attempt.pop();
    }
}

fn main() {
    let password_to_crack = "secret";  // Замените на ваш пароль
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"; // Используемые символы
    let max_password_length = 6;  // Максимальная длина пароля

    let mut current_attempt = String::new();
    bruteforce_recursive(password_to_crack, &mut current_attempt, charset, max_password_length);
}