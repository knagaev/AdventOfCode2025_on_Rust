use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};

pub fn local_main() -> io::Result<()> {
    let file_path = String::from("data/input_06.txt");
    let file = fs::File::open(&file_path)?;
    let mut reader = BufReader::new(file);

    // Day 1
    let lines: Vec<String> = (&mut reader).lines().collect::<Result<Vec<_>, _>>()?;

    let num_rows = lines.len();
    let op_row = num_rows - 1;
    let num_problems = lines[0].split_whitespace().count();

    let mut problems: Vec<Vec<u128>> = vec![Vec::with_capacity(op_row); num_problems];

    for line in lines[0..num_rows - 1].iter() {
        // Числовая строка — раскладываем по колонкам
        for (col_idx, word) in line.split_whitespace().enumerate() {
            problems[col_idx].push(word.parse::<u128>().expect("Ожидалось число"));
        }
    }
    let operators = lines[num_rows - 1]
        .split_whitespace()
        .map(|s| s.chars().next().expect("Ожидался оператор"))
        .collect::<Vec<char>>();

    let grand_total: u128 = problems
        .iter()
        .zip(operators.iter())
        .map(|(numbers, &op)| match op {
            '+' => numbers.iter().sum::<u128>(),
            '*' => numbers.iter().product::<u128>(),
            _ => panic!("Неизвестный оператор: {}", op),
        })
        .sum();

    println!("Grand total day 1: {}", grand_total);

    // Day 2
    reader.seek(SeekFrom::Start(0))?;

    let lines: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok) //фильтруем ошибки чтения
        .map(|line| line.chars().collect())
        .collect();

    if lines.is_empty() {
        return Ok(());
    }

    let cols = lines[0].len();

    let transposed_lines = (0..cols)
        .map(|c| lines.iter().map(|row| row[c]).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    //println!("transposed_lines: {:?}", transposed_lines);
    /*let _ = save_grid_to_file(
        &format!("data/transposed_{}", &file_path[5..]),
        &transposed_lines,
    );*/

    let mut grand_total: u64 = 0;
    let mut problem: Vec<u64> = vec![];
    let mut op: char = ' ';
    for el in transposed_lines {
        if el.iter().all(|ch| *ch == ' ') {
            let total = match op {
                '+' => problem.iter().sum::<u64>(),
                '*' => problem.iter().product::<u64>(),
                _ => panic!("Неизвестный оператор: {}", op),
            };
            grand_total += total;
            //println!("problem: {:?} {:?} {:?}", op, problem, total);
            problem.clear();
            continue;
        }
        let num = el[0..el.len() - 1]
            .iter()
            .filter(|&&ch| ch != ' ')
            .map(|&ch| (ch as u8) - b'0')
            .fold(0, |acc, digit| acc * 10 + digit as u64);

        problem.push(num);

        if let Some(&last_ch) = el.last()
            && last_ch != ' '
        {
            op = last_ch;
        }
    }
    let total = match op {
        '+' => problem.iter().sum::<u64>(),
        '*' => problem.iter().product::<u64>(),
        _ => panic!("Неизвестный оператор: {}", op),
    };
    grand_total += total;
    println!("Grand total day 2: {}", grand_total);
    Ok(())
}

fn save_grid_to_file(path: &str, grid: &Vec<Vec<char>>) -> std::io::Result<()> {
    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);

    for row in grid {
        let line: String = row.iter().collect();
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    Ok(())
}
