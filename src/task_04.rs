use std::fs;
use std::io;

use std::collections::VecDeque;

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_04.txt";
    let content = fs::read_to_string(FILE_PATH)?;

    let rolls_map: Vec<&str> = content.lines().collect();

    let accessible_rolls = count_accessible_rolls(&rolls_map);

    println!("accessible_rolls {:?}", accessible_rolls);

    let mut grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let total_removable_rolls = count_total_removable_rolls(&mut grid);

    println!("total_removable_rolls {:?}", total_removable_rolls);
    Ok(())
}

fn count_accessible_rolls(grid: &[&str]) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    let mut accessible_count = 0;

    // 8 соседей
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for r in 0..rows {
        let chars: Vec<char> = grid[r].chars().collect();
        for c in 0..cols {
            if chars[c] != '@' {
                continue;
            }

            let mut neighbors = 0;
            for &(dr, dc) in &directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                // границы
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let neighbor_row = &grid[nr as usize];

                    if neighbor_row.as_bytes()[nc as usize] == b'@' {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                accessible_count += 1;
            }
        }
    }

    accessible_count
}

// алгоритм имитирует BFS
fn count_total_removable_rolls(grid: &mut Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    let mut removed_count = 0;
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    // Вспомогательный массив, чтобы не добавлять один и тот же рулон в очередь дважды
    let mut in_queue = vec![vec![false; cols]; rows];

    // 8 соседей
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    // ищем возможных к удалению и добавляем в очередь
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' && count_active_neighbors(grid, r, c, &directions) < 4 {
                queue.push_back((r, c));
                in_queue[r][c] = true;
            }
        }
    }

    // пока есть в очереди для обработки
    while let Some((r, c)) = queue.pop_front() {
        // отмечаем как удаленный
        grid[r][c] = '.';
        removed_count += 1;

        // проверяем соседей не стал ли кто-нибудь удаляемым
        for &(dr, dc) in &directions {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nr_usize = nr as usize;
                let nc_usize = nc as usize;

                // если стоит рулон, который можно удалить, и он еще не в очереди
                if grid[nr_usize][nc_usize] == '@'
                    && !in_queue[nr_usize][nc_usize]
                    && count_active_neighbors(grid, nr_usize, nc_usize, &directions) < 4
                {
                    queue.push_back((nr_usize, nc_usize));
                    in_queue[nr_usize][nc_usize] = true;
                }
            }
        }
    }

    removed_count
}

// функция для подсчета активных соседей
fn count_active_neighbors(grid: &[Vec<char>], r: usize, c: usize, dirs: &[(i32, i32)]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    for &(dr, dc) in dirs {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0
            && nr < rows as i32
            && nc >= 0
            && nc < cols as i32
            && grid[nr as usize][nc as usize] == '@'
        {
            count += 1;
        }
    }
    count
}
