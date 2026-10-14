use std::fs;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
struct Machine {
    num_lights: usize,
    target_mask: u64,                 // битовая маска целевого состояния
    button_masks: Vec<u64>,           // битовые маски для каждой кнопки
    joltage: Vec<u32>,                // требования по счетчикам
    button_counters: Vec<Vec<usize>>, // кнопка → список счетчиков
}

fn parse_line(line: &str) -> Option<Machine> {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    if tokens.len() < 3 {
        return None; // минимум: диаграмма + 1 кнопка + joltage
    }

    // 1. Парсим диаграмму (первый токен)
    let diagram = &tokens[0]; // "[.##.]"
    let diagram_content = &diagram[1..diagram.len() - 1]; // ".##."

    let num_lights = diagram_content.len();
    let target_mask = diagram_content
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .fold(0u64, |mask, (i, _)| mask | (1 << i));

    // 2. Парсим кнопки (все токены кроме первого и последнего)
    let mut button_masks = Vec::new();
    let mut button_counters = Vec::new();
    for &token in &tokens[1..tokens.len() - 1] {
        // token вида "(1,3)" или "(0)"
        let content = &token[1..token.len() - 1]; // "1,3" или "0"

        let mask = content
            .split(',')
            .filter_map(|s| s.parse::<usize>().ok())
            .fold(0u64, |mask, i| mask | (1 << i));

        button_masks.push(mask);

        // Для Part Two — список индексов счетчиков
        let counters: Vec<usize> = content
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();
        button_counters.push(counters);
    }

    let joltage_token = tokens[tokens.len() - 1]; // "{3,5,4,7}"
    let joltage_content = &joltage_token[1..joltage_token.len() - 1]; // "3,5,4,7"
    let joltage: Vec<u32> = joltage_content
        .split(',')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect();

    Some(Machine {
        num_lights,
        target_mask,
        button_masks,
        joltage,
        button_counters,
    })
}

// решение одной машины
fn solve(machine: &Machine) -> Option<(Vec<Vec<u8>>, usize)> {
    let Machine {
        num_lights,
        target_mask,
        button_masks,
        joltage: _,
        button_counters: _,
    } = machine;

    let num_buttons = button_masks.len();

    // расширенная матрица: num_lights строк, каждая u64 - битовая маска
    // Бит j (j < n) = A[i][j]
    // Бит n = b[i]
    let mut augmented: Vec<u64> = vec![0u64; *num_lights];

    // Для каждой кнопки заполняем соответствующий столбец
    for (btn_idx, &btn_mask) in button_masks.iter().enumerate() {
        // btn_mask показывает, какие индикаторы переключает эта кнопка
        // Заполняем столбец btn_idx в матрице
        /*for light in 0..num_lights {
            if (btn_mask >> light) & 1 == 1 {
                augmented[light] |= 1 << btn_idx;
            }
        }*/
        for (light_idx, light) in augmented.iter_mut().enumerate() {
            if (btn_mask >> light_idx) & 1 == 1 {
                *light |= 1 << btn_idx;
            }
        }
    }

    // Добавляем правую часть b
    for (light_idx, light) in augmented.iter_mut().enumerate() {
        if (target_mask >> light_idx) & 1 == 1 {
            *light |= 1 << num_buttons;
        }
    }

    let mut pivot_row = 0;
    let mut pivot_cols: Vec<usize> = Vec::new(); // отслеживаем ведущие колонки

    for col in 0..num_buttons {
        // Ищем строку с 1 в текущей колонке
        let mut found_row_idx = None;
        for (row_idx, row) in augmented.iter().enumerate().skip(pivot_row) {
            if (*row >> col) & 1 == 1 {
                found_row_idx = Some(row_idx);
                break;
            }
        }

        if let Some(found_row_idx) = found_row_idx {
            // Меняем местами строки
            augmented.swap(pivot_row, found_row_idx);

            // Обнуляем все остальные строки в этой колонке
            for row in 0..augmented.len() {
                if row != pivot_row && (augmented[row] >> col) & 1 == 1 {
                    augmented[row] ^= augmented[pivot_row];
                }
            }

            pivot_cols.push(col);
            pivot_row += 1;
        }
        // Если не нашли - колонка свободная, пропускаем
    }

    // проверка на совместность
    let left_mask = (1u64 << num_buttons) - 1; // Маска для битов переменных

    for row in augmented.iter().skip(pivot_row) {
        // если все коэффициенты нулевые
        if (row & left_mask) == 0 {
            // а свободный член ненулевой (0 * x = 1)
            if (row >> num_buttons) & 1 == 1 {
                return None; // несовместна
            }
        }
    }

    let mut free_vars: Vec<usize> = Vec::new();
    for col in 0..num_buttons {
        if !pivot_cols.contains(&col) {
            free_vars.push(col);
        }
    }

    let num_free = free_vars.len();
    let total_combinations = 1usize << num_free; // 2^k

    let solutions: Vec<Vec<u8>> = (0..total_combinations)
        .map(|mask| {
            let mut solution = vec![0u8; num_buttons];

            // Заполняем свободные переменные
            for (i, &free_col) in free_vars.iter().enumerate() {
                solution[free_col] = ((mask >> i) & 1) as u8;
            }

            // Вычисляем базисные переменные
            for (i, &pivot_col) in pivot_cols.iter().enumerate() {
                let row = augmented[i];
                let mut val = ((row >> num_buttons) & 1) as u8;

                // XOR-сумма по всем переменным в строке, кроме ведущей
                for (col_idx, col) in solution.iter().enumerate().take(num_buttons) {
                    if col_idx != pivot_col && ((row >> col_idx) & 1) == 1 {
                        val ^= col;
                    }
                }
                solution[pivot_col] = val;
            }

            // Считаем сумму нажатий
            solution
        })
        .collect();

    let min_presses = solutions
        .iter()
        .map(|x| x.iter().sum::<u8>())
        .min()
        .unwrap_or(0) as usize; // Если решений нет (хотя мы уже проверили совместность), берем 0

    println!(
        "target_mask: {:?} min_presses: {:?} solutions {:?}",
        target_mask, min_presses, solutions
    );
    Some((solutions, min_presses))
}

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_10_sample2.txt";
    let file = fs::File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();

    let mut machines: Vec<Machine> = Vec::new();
    for line in lines {
        let line = line?;
        if let Some(machine) = parse_line(&line) {
            machines.push(machine);
        }
    }

    let result = machines.iter().try_fold(0usize, |acc, machine| {
        solve(machine).map(|(_, val)| acc + val)
    });
    println!("Part One: {:?} ", result);

    let result2 = machines
        .iter()
        .try_fold(0u32, |acc, m| solve_part2(m).map(|v| acc + v));

    println!("Part Two: {:?}", result2);

    Ok(())
}

fn solve_part2(machine: &Machine) -> Option<u32> {
    Some(0)
}
