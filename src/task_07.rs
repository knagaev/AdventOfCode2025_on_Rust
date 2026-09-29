use std::fs::{self};
use std::io::{self};

/*
fn process_beam_row(beams: &str, splitters: &str) -> (String, u64) {
    let beam_chars: Vec<char> = beams.chars().collect();
    let split_chars: Vec<char> = splitters.chars().collect();
    let len = beam_chars.len();

    let mut new_beams = vec!['.'; len];
    let mut split_times: u64 = 0;

    for i in 0..len {
        // Правило 1 и проверка наличия луча
        if beam_chars[i] == '|' {
            if split_chars[i] == '^' {
                // Правило 3: Луч встречает сплиттер
                split_times += 1;
                // Создаем луч слева, если клетка существует
                if i > 0 {
                    new_beams[i - 1] = '|';
                }

                // Создаем луч справа, если клетка существует
                if i + 1 < len {
                    new_beams[i + 1] = '|';
                }

                // В текущей клетке луча нет (остается '.')
            } else {
                // Правило 2: Луч есть, сплиттера нет -> сохраняем луч
                new_beams[i] = '|';
            }
        }
        // Если beam_chars[i] == '.', то new_beams[i] уже равно '.' по умолчанию
    }

    (new_beams.into_iter().collect(), split_times)
}

pub fn local_main() -> io::Result<()> {
    let file_path = String::from("data/input_07.txt");
    let file = fs::File::open(&file_path)?;
    let reader = BufReader::new(file);

    // Day 1
    let mut lines = reader.lines();
    let first_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Нет первой строки!"))??;
    //let tachyon_status = convert_line_to_array(&first_line);
    let mut tachyon_status = first_line.replace("S", "|");
    println!("{:?}", tachyon_status);
    let mut total_split_times: u64 = 0;
    let mut split_times: u64 = 0;
    for line in lines {
        let line = line?;
        println!("line   _status {:?}", line);
        //let splitter_line = convert_line_to_array(&line);
        (tachyon_status, split_times) = process_beam_row(&tachyon_status, &line);
        total_split_times += split_times;
        println!("tachyon_status {:?}", tachyon_status);
    }

    let grand_total = tachyon_status.chars().filter(|&c| c == '|').count();

    println!("Beams split times: {}", total_split_times);
    Ok(())
}

*/

pub fn local_main() -> io::Result<()> {
    let content = fs::read_to_string("data/input_07_sample.txt")?;

    // Day 1
    let cols = content.lines().next().unwrap_or("").len();

    // Фильтруем переносы строк и собираем всё в один плоский вектор
    let manifold: Vec<char> = content.lines().flat_map(|line| line.chars()).collect();

    if manifold.is_empty() {
        return Ok(());
    }

    let timelines = count_timelines(&manifold, cols).unwrap_or(0);

    println!("Beam timelines: {}", timelines);
    Ok(())
}

#[derive(Debug)]
enum Direction {
    Down,
    Up,
}

fn count_timelines(manifold: &[char], cols: usize) -> Option<u64> {
    let mut timelines: u64 = 0;
    let grid = manifold;

    //let rows = grid.len() / cols;

    let pos = grid[0..cols].iter().position(|&c| c == 'S')?;
    //println!("{:?} {:?}", pos/cols, pos%cols);
    let mut movement = Direction::Down;
    let mut stack: Vec<usize> = Vec::new();
    stack.push(pos);
    //let memo: Vec<u64>
    while !stack.is_empty() {
        let pos = *stack.last()?;
        let pos = if pos >= grid.len() - cols {
            timelines += 1;
            stack.pop();
            let pos = *stack.last()?;
            movement = Direction::Up;
            pos
        } else {
            pos
        };

        println!("{:?} {:?} {:?} {:?}", movement, pos, pos / cols, pos % cols);
        match movement {
            Direction::Down => match grid[pos + cols] {
                '.' => stack.push(pos + cols),
                '^' => {
                    // если сплиттер в крайней левой колонке, то запускаем луч по правой ветке
                    if pos + cols % cols == 0 {
                        stack.push(pos + cols + 1);
                    } else {
                        // иначе запускаем луч по левой ветке
                        stack.push(pos + cols - 1);
                    }
                }
                _ => panic!("Неизвестный символ"),
            },
            Direction::Up => match grid[pos + cols] {
                '^' => {
                    // если крайняя колонка,
                    // значит опускаться уже некуда, продолжаем подниматься
                    if pos % cols == 0 || pos % cols == cols - 1 {
                        stack.pop();
                    } else {
                        // иначе значит дальше идём по правой ветке вниз
                        stack.push(pos + cols + 1);
                        movement = Direction::Down
                    }
                }
                _ => {
                    stack.pop();
                }
            },
        }
    }

    Some(timelines)
}
