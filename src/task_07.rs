use std::collections::HashSet;
use std::fs;
use std::io::{self, BufWriter, Write};

#[derive(Debug, Default, Clone)]
pub struct Splitter {
    pub left_splitter: Option<usize>,  // Индекс левого потомка в векторе
    pub right_splitter: Option<usize>, // Индекс правого потомка в векторе
    pub memo: u64,                     //  Кэш: количество траекторий от этого сплиттера до конца
}

pub fn local_main() -> io::Result<()> {
    let content = fs::read_to_string("data/input_07.txt")?;

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
                //println!("Сплиттер найден {} {}", i, j);
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

    let splits = count_splits(first_splitter_idx, &splitters);
    println!("Splits: {}", splits);

    let timelines = count_timelines(first_splitter_idx, &mut splitters, cols);
    println!("Timelines: {}", timelines);

    Ok(())
}

fn save_splitters_to_file(path: &str, splitters: &[Option<Splitter>]) -> std::io::Result<()> {
    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);

    for s in splitters {
        writeln!(writer, "{:?}", s)?;
    }

    writer.flush()?;
    Ok(())
}

fn count_splits(start_splitter_idx: usize, splitters: &[Option<Splitter>]) -> u64 {
    let mut splits: HashSet<usize> = HashSet::from([start_splitter_idx]);

    let mut front: HashSet<usize> = HashSet::from([start_splitter_idx]);
    while !front.is_empty() {
        splits.extend(&front);
        front = front
            .iter()
            .filter_map(|&idx| splitters[idx].as_ref())
            .flat_map(|s| s.left_splitter.into_iter().chain(s.right_splitter))
            .collect::<HashSet<_>>();
    }

    splits.len() as u64
}

fn count_timelines(
    start_splitter_idx: usize,
    splitters: &mut [Option<Splitter>],
    cols: usize,
) -> u64 {
    let mut stack = vec![start_splitter_idx];
    //println!("start_idx {}", start_splitter_idx);

    while let Some(&splitter_idx) = stack.last() {
        // получаем ссылку на текущий сплиттер
        let splitter_opt = &splitters[splitter_idx];
        //println!("splitter {:?}", splitter_opt);

        if splitter_opt.is_none() {
            panic!("No splitter at {}", splitter_idx);
        }

        let splitter = splitter_opt.as_ref().unwrap();
        let col = splitter_idx % cols;
        //println!("Splitter {} {}", splitter_idx / cols, col);

        // левый потомок
        if let Some(left_idx) = splitter.left_splitter {
            //println!("Проверяем левого {:?}", left_idx);
            if let Some(ref left_splitter) = splitters[left_idx] {
                //println!("Получаем левого {:?}", left_splitter);
                if left_splitter.memo == 0 {
                    stack.push(left_idx);
                    continue;
                }
            }
        }

        // правый потомок
        if let Some(right_idx) = splitter.right_splitter {
            //println!("Проверяем правого {:?}", right_idx);
            if let Some(ref right_splitter) = splitters[right_idx] {
                //println!("Получаем правого {:?}", right_splitter);
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

        //println!("Splitter {} {} paths {}", splitter_idx / cols, col, paths);

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
