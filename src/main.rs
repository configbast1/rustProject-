use std::fs::File;
use std::io::Write;

fn main() {
    let numbers = vec![10.0, 20.0, 30.0, 40.0, 50.0];

    let sum: f64 = numbers.iter().sum();
    let count = numbers.len();
    let average = if count > 0 {
        sum / count as f64
    } else {
        0.0
    };

    let output = format!(
        "# 📊 Статистика\n\n\
        Числа: {:?}\n\
        Сумма: {}\n\
        Количество: {}\n\
        Среднее: {:.2}\n",
        numbers, sum, count, average
    );

    let mut file = File::create("STATS.md").expect("Не удалось создать файл");
    file.write_all(output.as_bytes())
        .expect("Не удалось записать в файл");

    println!("STATS.md обновлён!");
}
