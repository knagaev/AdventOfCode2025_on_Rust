use std::fs;
use std::io::{self, BufRead, BufReader};

pub fn local_main() -> io::Result<()> {
    let file = fs::File::open("data/input_06.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;

    if lines.is_empty() {
        return Ok(());
    }

    let num_rows = lines.len();
    let op_row = num_rows - 1;

    // Определяем количество задач по первой строке
    let num_problems = lines[0].split_whitespace().count();

    // Создаём "пустую" структуру: N векторов (по одному на задачу)
    let mut problems: Vec<Vec<u128>> = vec![Vec::with_capacity(op_row); num_problems];
    //let mut operators: Vec<char> = Vec::with_capacity(num_problems);

    // Парсим строки по одной, сразу раскладывая числа по колонкам
    for line in lines[0..num_rows - 1].iter() {
        // Числовая строка — раскладываем по колонкам
        for (col_idx, word) in line.split_whitespace().enumerate() {
            problems[col_idx].push(word.parse::<u128>().expect("Ожидалось число"));
        }
    }
    // Последняя строка — операторы
    let operators = lines[num_rows - 1]
        .split_whitespace()
        .map(|s| s.chars().next().expect("Ожидался оператор"))
        .collect::<Vec<char>>();

    // Теперь для каждой задачи у нас уже готовы все числа — просто применяем оператор
    let grand_total: u128 = problems
        .iter()
        .zip(operators.iter())
        .map(|(numbers, &op)| match op {
            '+' => numbers.iter().sum::<u128>(),
            '*' => numbers.iter().product::<u128>(),
            _ => panic!("Неизвестный оператор: {}", op),
        })
        .sum();

    println!("Grand total: {}", grand_total);
    Ok(())
}
