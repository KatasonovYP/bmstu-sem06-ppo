use clap::{Arg, Command};

fn main() {
    let matches = Command::new("cli")
        .version("0.1.0")
        .author("Ваше имя")
        .about("Простое CLI приложение с поддержкой аргументов")
        // Позиционные аргументы
        .arg(
            Arg::new("input")
                .help("Входной файл")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("Выходной файл")
                .required(false)
                .index(2),
        )
        // Именованные аргументы
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Включить подробный вывод")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Количество повторений")
                .default_value("1"),
        )
        .get_matches();

    // Получаем значения позиционных аргументов
    let input = matches.get_one::<String>("input").unwrap();
    println!("Входной файл: {}", input);

    if let Some(output) = matches.get_one::<String>("output") {
        println!("Выходной файл: {}", output);
    } else {
        println!("Выходной файл не указан");
    }

    // Получаем значения именованных аргументов
    let verbose = matches.get_flag("verbose");
    println!("Подробный вывод: {}", verbose);

    let count = matches.get_one::<String>("count").unwrap();
    println!("Количество повторений: {}", count);
}
