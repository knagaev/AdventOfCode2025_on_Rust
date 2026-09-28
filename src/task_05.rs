use std::fs;
use std::io::{self, BufRead, BufReader};

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_05.txt";
    let file = fs::File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    // сначала читаем диапазоны
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line_result in lines.by_ref() {
        let line = line_result?;
        if line.trim().is_empty() {
            break;
        }
        if let Some((start_str, end_str)) = line.trim().split_once('-') {
            let start = start_str.parse::<u64>().ok();
            let end = end_str.parse::<u64>().ok();
            if let (Some(s), Some(e)) = (start, end) {
                ranges.push((s, e));
            }
        }
    }

    // считываем ID
    let mut fresh_count = 0;
    for line_result in lines {
        let line = line_result?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Ok(id) = trimmed.parse::<u64>()
            && is_fresh(id, &ranges)
        {
            fresh_count += 1;
        }
    }

    println!("Количество свежих ингредиентов: {}", fresh_count);

    //for range in ranges
    let merged_ranges = merge_ranges(ranges);
    let possible_fresh_count: u64 = merged_ranges
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum();

    println!(
        "Количество возможных свежих ингредиентов: {}",
        possible_fresh_count
    );

    Ok(())
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    // проверка попаданиия в диапазон
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

// слияние интервалов: сортируем по началу, первый активный
// если начало следующего интервала не больше конца активного, расширяем активный до конца следующего
// если больше - следующий становится активным
// передача по значению с владением - старый вектор больше не нужен
fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return vec![];
    }

    // сортируем по началу
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged = Vec::new();
    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for &(start, end) in &ranges[1..] {
        if start <= current_end + 1 {
            current_end = current_end.max(end);
        } else {
            merged.push((current_start, current_end));
            current_start = start;
            current_end = end;
        }
    }

    // последний активный интервал
    merged.push((current_start, current_end));

    merged
}
