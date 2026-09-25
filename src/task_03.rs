use std::fs;
use std::io::{self, BufRead, BufReader};

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_03.txt";
    let file = fs::File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    let mut total_max_joltage = 0_u32;
    let mut total_max_joltage_12 = 0_u64;
    let mut total_max_joltage_u128 = 0_u128;

    for bank in reader.lines() {
        let bank = bank?; // line здесь имеет тип Result<String>, распаковываем его
        //println!("Банк>: {}", bank);
        total_max_joltage += get_max_joltage(&bank).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "get_max_joltage не вернула результат!",
            )
        })?;

        total_max_joltage_12 += get_max_joltage_12(&bank).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "get_max_joltage_12 не вернула результат!",
            )
        })?;

        total_max_joltage_u128 += get_max_joltage_u128(&bank, 12).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "get_max_joltage_12 не вернула результат!",
            )
        })?;
    }

    println!("total_max_joltage {:?}", total_max_joltage);
    println!("total_max_joltage_12 {:?}", total_max_joltage_12);
    println!("total_max_joltage_u128 {:?}", total_max_joltage_u128);
    Ok(())
}

fn get_max_joltage(bank: &str) -> Option<u32> {
    let (left_index, left_d) = bank[..bank.len() - 1]
        .char_indices()
        .rev()
        .map(|(index, ch)| (index, ch.to_digit(10).unwrap()))
        .max_by_key(|&(_, d)| d)
        .expect("Ожидалась хотя бы одна цифра в левой части");

    let right_d = bank[left_index + 1..]
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .max()
        .expect("Ожидалась хотя бы одна цифра в левой части");

    /*
    println!(
        "{:?} {:?} {:?} {:?}",
        left_index,
        left_d,
        right_d,
        10 * left_d + right_d
    );*/

    Some(10 * left_d + right_d)
}

fn get_max_joltage_12(bank: &str) -> Option<u64> {
    let mut max_joltage = 0_u64;
    let mut cur_pos = 0_usize;

    for i in (0..=11).rev() {
        let (cur_index, cur_d) = bank[cur_pos..bank.len() - i]
            .char_indices()
            .rev()
            .map(|(index, ch)| (index, ch.to_digit(10).unwrap()))
            .max_by_key(|&(_, d)| d)
            .expect("Ожидалась хотя бы одна цифра");

        //println!("cur_index {:?}, cur_d {:?}", cur_index, cur_d);
        max_joltage += (cur_d as u64) * 10_u64.pow(i as u32);
        cur_pos += cur_index + 1;
    }

    //println!("max_joltage {:?}", max_joltage);
    Some(max_joltage)
}

/// каноническая версия поиска максимальной подпоследовательности со стеком
fn get_max_joltage_u128(s: &str, k: usize) -> Option<u128> {
    println!("{}", s);
    let n = s.len(); // Для ASCII-цифр len() == кол-ву символов
    if k > n || k == 0 {
        return None;
    }

    let to_remove = n - k;

    let mut stack: Vec<u8> = Vec::with_capacity(k);
    let mut removed = 0;

    for &b in s.as_bytes() {
        while removed < to_remove && !stack.is_empty() && *stack.last().unwrap() < b {
            stack.pop();
            removed += 1;
            //println!("stack {:?}", String::from_utf8_lossy(&stack));
        }
        stack.push(b);
        //println!("stack {:?}", String::from_utf8_lossy(&stack));
    }

    // обрезаем до нужной длины
    stack.truncate(k);

    // преобразование в число
    let mut result: u128 = 0;
    for &b in &stack {
        result = result * 10 + (b - b'0') as u128;
    }

    Some(result)
}
