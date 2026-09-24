use std::collections::BTreeSet;
use std::fs;
use std::io::{self, BufRead, BufReader};

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_02.txt";
    let file = fs::File::open(FILE_PATH)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let ranges: Vec<(u128, u128)> = line
        .trim()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (start_str, end_str) = s.split_once('-').unwrap();
            (
                start_str.parse::<u128>().unwrap(),
                end_str.parse::<u128>().unwrap(),
            )
        })
        .collect();

    if ranges.is_empty() {
        println!("Нет диапазонов для обработки");
        return Ok(());
    }

    let max_id = ranges.iter().map(|(_, e)| e).max().unwrap();

    // генерируем все "невалидные" ID для первой задачи
    let invalid_ids = generate_invalid_ids(*max_id);

    let mut total_sum: u128 = 0;

    for (start, end) in &ranges {
        // находим все invalid_ids в диапазоне [start, end]
        // так как invalid_ids отсортированы (BTreeSet), можно использовать range
        //println!("start {:?}, end {:?}", start, end);
        for &id in invalid_ids.range(*start..=*end) {
            //println!("id  {:?}", id);
            total_sum += id;
        }
    }
    println!("Сумма всех невалидных ID в первой задаче: {}", total_sum);

    // генерируем все "невалидные" ID для второй задачи
    let invalid_ids_2 = generate_invalid_ids_2(*max_id);

    //for id in &invalid_ids_2 {
    //    println!("{}", id);
    //}
    let mut total_sum: u128 = 0;

    for (start, end) in &ranges {
        // находим все invalid_ids в диапазоне [start, end]
        // так как invalid_ids отсортированы (BTreeSet, а не HashSet), можно использовать range
        for &id in invalid_ids_2.range(*start..=*end) {
            //println!("id: {}", id);
            total_sum += id;
        }
    }
    println!("Сумма всех невалидных ID во второй задаче: {}", total_sum);

    Ok(())
}

fn generate_invalid_ids(max_id: u128) -> BTreeSet<u128> {
    let mut ids = BTreeSet::new(); // упорядоченный set для поиска по range

    // максимальная длина числа в десятичной записи
    let max_len = if max_id == 0 { 1 } else { max_id.ilog10() + 1 };

    // паттерн может быть длиной от 1 до max_len / 2 (так как повтор минимум 2 раза)
    for pattern_len in 1..=max_len / 2 {
        let start_pattern = 10_u128.pow(pattern_len - 1);
        let end_pattern = 10_u128.pow(pattern_len);

        for pattern in start_pattern..end_pattern {
            ids.insert(pattern * (1 + 10_u128.pow(pattern_len)));
        }
    }

    ids
}

fn generate_invalid_ids_2(max_id: u128) -> BTreeSet<u128> {
    let mut ids = BTreeSet::new(); // упорядоченный set для поиска по range

    // максимальная длина числа в десятичной записи
    let max_len = if max_id == 0 { 1 } else { max_id.ilog10() + 1 };
    //println!("max_id {}б max_len {}", max_id, max_len);

    // паттерн может быть длиной от 1 до max_len / 2 (так как повтор минимум 2 раза)
    for pattern_len in 1..=max_len / 2 {
        let start_pattern = 10_u128.pow(pattern_len - 1);
        let end_pattern = 10_u128.pow(pattern_len);

        for pattern in start_pattern..end_pattern {
            let mut current_num = pattern;

            // повторяем паттерн, пока число не станет слишком большим
            loop {
                let multiplier = 10_u128.pow(pattern_len);

                // проверка на переполнение перед умножением
                if current_num > u128::MAX / multiplier {
                    break;
                }
                current_num = current_num * multiplier + pattern;

                if current_num > max_id {
                    break;
                }

                //println!("current_num {}", current_num);
                ids.insert(current_num);
            }
        }
    }

    ids
}
