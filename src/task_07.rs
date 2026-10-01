use std::collections::{HashSet, VecDeque};
use std::fs;
use std::io::{self, BufWriter, Write};

#[derive(Debug, Default, Clone)]
pub struct Splitter {
    pub left_splitter: Option<usize>,  // Индекс левого потомка в векторе
    pub right_splitter: Option<usize>, // Индекс правого потомка в векторе
    pub memo: u64,                     //  Кэш: количество траекторий от этого сплиттера до конца
}

pub fn local_main() -> io::Result<()> {
    let content = fs::read_to_string("data/input_07_sample.txt")?;

    // Day 1
    let first_line = content.lines().next().unwrap_or("");
    let enter_pos = first_line.find('S').unwrap_or(0);
    let cols = first_line.len();

    // Фильтруем переносы строк и собираем всё в один плоский вектор
    let manifold: Vec<char> = content.lines().flat_map(|line| line.chars()).collect();
    let rows = manifold.len() / cols;

    if manifold.is_empty() {
        return Ok(());
    }

    let mut splitters: Vec<Option<Splitter>> = vec![None; manifold.len()];

    for j in 0..cols {
        for i in 0..rows {
            if manifold[i * cols + j] == '^' {
                println!("Сплиттер найден {} {}", i, j);
                let splitter_index = i * cols + j;
                let splitter = splitters[splitter_index].get_or_insert_with(Splitter::default);
                if j > 0 {
                    // левая ветка
                    for k in i + 1..rows {
                        if manifold[k * cols + j - 1] == '^' {
                            let left_child_index = k * cols + j - 1;
                            splitter.left_splitter = Some(left_child_index);
                            break;
                        }
                    }
                }
                if j < cols - 1 {
                    // правая ветка
                    for k in i + 1..rows {
                        if manifold[k * cols + j + 1] == '^' {
                            let right_child_index = k * cols + j + 1;
                            splitter.right_splitter = Some(right_child_index);
                            break;
                        }
                    }
                }
            }
        }
    }

    let _ = save_splitters_to_file("test.txt", &splitters);

    let first_splitter_row = splitters[enter_pos..]
        .iter()
        .step_by(cols)
        .position(|x| x.is_some())
        .unwrap();
    let first_splitter_idx = first_splitter_row * cols + enter_pos;

    let beams = count_beams(first_splitter_idx, &splitters, cols);
    let timelines = count_timelines(first_splitter_idx, &mut splitters, cols);

    /*

        // Day 1
        let file_path = String::from("data/input_07.txt");
        let file = fs::File::open(&file_path)?;
        let reader = BufReader::new(file);

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
    */
    println!("Beams: {}", beams);
    println!("Timelines: {}", timelines);
    Ok(())
}

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


pub fn local_main() -> io::Result<()> {
    let content = fs::read_to_string("data/input_07_sample.txt")?;

    // Day 1
    let cols = content.lines().next().unwrap_or("").len();

    // Фильтруем переносы строк и собираем всё в один плоский вектор
    let manifold: Vec<char> = content.lines().flat_map(|line| line.chars()).collect();
    //let _ = save_grid_to_file("test.txt", &manifold, cols);

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
    //    let _ = save_grid_to_file("test.txt", grid, cols);

    //let rows = grid.len() / cols;

    let mut pos = grid[0..cols].iter().position(|&c| c == 'S')?;
    //println!("{:?} {:?}", pos/cols, pos%cols);
    let mut movement = Direction::Down;
    let mut stack: Vec<usize> = Vec::new();
    stack.push(pos);
    let mut memo: Vec<u64> = vec![0; grid.len()];

    while !stack.is_empty() {
        pos = *stack.last()?;
        if pos >= grid.len() - cols {
            timelines += 1;
            println!(
                "касание {:?} {:?} {:?} {:?}",
                movement,
                pos / cols,
                pos % cols,
                timelines
            );
            movement = Direction::Up;
            stack.pop();
            pos = *stack.last()?;
        }

        println!("{:?} {:?} {:?}", movement, pos / cols, pos % cols);
        match movement {
            Direction::Down => {
                if memo[pos] > 0 {
                    println!(
                        "Используется {:?} {:?} {:?} memo {:?}",
                        movement,
                        pos / cols,
                        pos % cols,
                        memo[pos]
                    );
                    timelines += memo[pos];
                    movement = Direction::Up;
                    stack.pop();
                } else {
                    match grid[pos + cols] {
                        '.' => stack.push(pos + cols),
                        '^' => {
                            // если сплиттер в крайней левой колонке, то запускаем луч по правой ветке
                            if pos % cols == 0 {
                                stack.push(pos + cols + 1);
                            } else {
                                // иначе запускаем луч по левой ветке
                                stack.push(pos + cols - 1);
                            }
                        }
                        _ => panic!("Неизвестный символ"),
                    }
                }
            }
            Direction::Up => match grid[pos + cols] {
                '^' => {
                    // если крайняя колонка или уже обрабатывали обе ветки,
                    // значит опускаться уже не надо, продолжаем подниматься
                    if pos % cols == 0 || pos % cols == cols - 1 || memo[pos] > 0 {
                        stack.pop();
                    } else {
                        // иначе значит дальше идём по правой ветке вниз
                        stack.push(pos + cols + 1);
                        movement = Direction::Down
                    }
                }
                _ => {
                    stack.pop();
                    if stack.is_empty() {
                        continue;
                    }
                    let new_pos = *stack.last()?;
                    // если сплиттер и обработка траекторий закончена
                    if grid[new_pos + cols] == '^'
                        && (
                            new_pos % cols == 0 // идем вверх по крайней левой колонке
                        || new_pos % cols == cols - 1  // идем вверх по крайней правой колонке
                        || new_pos < pos - cols
                            // идем вверх и вернулись из правой ветки
                        )
                    {
                        memo[new_pos] = timelines;
                        println!(
                            "Подсчитано {:?} {:?} {:?} memo {:?}",
                            movement,
                            new_pos / cols,
                            new_pos % cols,
                            memo[new_pos]
                        );
                    }
                }
            },
        }
    }

    println!("Beam timelines in function!");
    println!("Beam timelines in function: {:?}", timelines);
    Some(timelines)
}


fn save_grid_to_file(path: &str, grid: &[Option<Splitter>], cols: usize) -> std::io::Result<()> {
    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);

    for i in 0..grid.len() / cols {
        let line = &grid[i * cols..(i + 1) * cols];
        writeln!(writer, "{:?}", line)?;
    }

    writer.flush()?;
    Ok(())
}
*/
fn save_splitters_to_file(path: &str, splitters: &[Option<Splitter>]) -> std::io::Result<()> {
    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);

    for s in splitters {
        writeln!(writer, "{:?}", s)?;
    }

    writer.flush()?;
    Ok(())
}

fn count_beams(start_splitter_idx: usize, splitters: &[Option<Splitter>], cols: usize) -> u64 {
    let mut result_beams: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::from([start_splitter_idx]);

    while let Some(splitter_idx) = queue.pop_front() {
        let splitter_col = splitter_idx % cols;
        let splitter_opt = &splitters[splitter_idx];

        if splitter_opt.is_none() {
            panic!("No splitter at {}", splitter_idx);
        }

        let splitter = splitter_opt.as_ref().unwrap();

        if let Some(left_idx) = splitter.left_splitter {
            println!("Проверяем левого {:?}", left_idx);
            queue.push_back(left_idx);
        } else {
            if splitter_col > 0 {
                result_beams.insert(splitter_col - 1);
            }
        }

        if let Some(right_idx) = splitter.right_splitter {
            println!("Проверяем правого {:?}", right_idx);
            queue.push_back(right_idx);
        } else {
            if splitter_col < cols - 1 {
                result_beams.insert(splitter_col + 1);
            }
        }
    }

    result_beams.len() as u64
}

fn count_timelines(
    start_splitter_idx: usize,
    splitters: &mut [Option<Splitter>],
    cols: usize,
) -> u64 {
    let mut stack = vec![start_splitter_idx];
    println!("start_idx {}", start_splitter_idx);

    while let Some(&splitter_idx) = stack.last() {
        // Получаем ссылку на текущий сплиттер
        let splitter_opt = &splitters[splitter_idx];
        println!("splitter {:?}", splitter_opt);

        if splitter_opt.is_none() {
            panic!("No splitter at {}", splitter_idx);
        }

        let splitter = splitter_opt.as_ref().unwrap();
        let col = splitter_idx % cols;
        println!("Splitter {} {}", splitter_idx / cols, col);

        // левый потомок
        if let Some(left_idx) = splitter.left_splitter {
            println!("Проверяем левого {:?}", left_idx);
            if let Some(ref left_splitter) = splitters[left_idx] {
                println!("Получаем левого {:?}", left_splitter);
                if left_splitter.memo == 0 {
                    stack.push(left_idx);
                    continue;
                }
            }
        }

        // правый потомок
        if let Some(right_idx) = splitter.right_splitter {
            println!("Проверяем правого {:?}", right_idx);
            if let Some(ref right_splitter) = splitters[right_idx] {
                println!("Получаем правого {:?}", right_splitter);
                if right_splitter.memo == 0 {
                    stack.push(right_idx);
                    continue;
                }
            }
        }

        // подсчет траекторий от данного сплиттера
        let mut paths = 0u64;

        // траектории слева
        if let Some(left_idx) = splitter.left_splitter {
            if let Some(ref left_s) = splitters[left_idx] {
                paths += left_s.memo;
            }
        } else {
            // Нет левого потомка - луч выходит если
            if col > 0 {
                // сплиттер находится не в крайней колонке слева
                paths += 1;
            }
        }

        // траектории справа
        if let Some(right_idx) = splitter.right_splitter {
            if let Some(ref right_s) = splitters[right_idx] {
                paths += right_s.memo;
            }
        } else {
            // Нет правого потомка - луч выходит если
            if col < cols - 1 {
                // сплиттер находится не в крайней колонке справа
                paths += 1;
            }
        }

        println!("Splitter {} {} paths {}", splitter_idx / cols, col, paths);

        // запись в memo
        if let Some(ref mut s_mut) = splitters[splitter_idx] {
            s_mut.memo = paths;
        }

        // возврат вверх из обработанного сплиттера
        stack.pop();
    }

    // результат - memo стартового сплиттера
    if let Some(ref s) = splitters[start_splitter_idx] {
        s.memo
    } else {
        1
    }
}
