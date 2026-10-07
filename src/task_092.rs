use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_09.txt"; // измените на ваш файл
    let file = fs::File::open(FILE_PATH)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut red_tiles: Vec<Point> = Vec::new();
    for line in lines {
        let coord = line?
            .trim()
            .split(',')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        red_tiles.push(Point {
            x: coord[0],
            y: coord[1],
        });
    }

    let largest_area = solve(red_tiles);
    println!("the largest area of any rectangle: {largest_area}");
    Ok(())
}

fn solve(red_tiles: Vec<Point>) -> u64 {
    // Шаг 1: Собрать все красные плитки
    let red_set: HashSet<Point> = red_tiles.iter().copied().collect();

    // Шаг 2: Найти все зелёные плитки на границе
    let green_boundary = find_green_boundary(&red_tiles);

    // Шаг 3: Объединить красные и граничные зелёные плитки
    let mut valid_tiles: HashSet<Point> = red_set.clone();
    valid_tiles.extend(green_boundary.iter());

    // Шаг 4: Найти min/max координаты
    let min_x = red_tiles.iter().map(|p| p.x).min().unwrap();
    let max_x = red_tiles.iter().map(|p| p.x).max().unwrap();
    let min_y = red_tiles.iter().map(|p| p.y).min().unwrap();
    let max_y = red_tiles.iter().map(|p| p.y).max().unwrap();

    println!(
        "Bounding box: ({}, {}) to ({}, {})",
        min_x, min_y, max_x, max_y
    );

    // Шаг 5: Найти все внутренние зелёные плитки
    let green_interior = find_green_interior(&valid_tiles, min_x, max_x, min_y, max_y);
    valid_tiles.extend(green_interior);

    println!("Total valid tiles: {}", valid_tiles.len());

    // Шаг 6: Создать структуру для быстрой проверки строк и столбцов
    // Для каждой строки храним множество x-координат допустимых точек
    let mut rows: std::collections::HashMap<u64, BTreeSet<u64>> = std::collections::HashMap::new();
    // Для каждого столбца храним множество y-координат допустимых точек
    let mut cols: std::collections::HashMap<u64, BTreeSet<u64>> = std::collections::HashMap::new();

    for &point in &valid_tiles {
        rows.entry(point.y)
            .or_insert_with(BTreeSet::new)
            .insert(point.x);
        cols.entry(point.x)
            .or_insert_with(BTreeSet::new)
            .insert(point.y);
    }

    // Шаг 7: Перебрать пары красных точек, но с оптимизацией
    // Сортируем красные точки по площади потенциального прямоугольника
    let mut max_area = 0;

    // Оптимизация: проверяем только пары, которые могут дать площадь больше текущей максимум
    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let p1 = red_tiles[i];
            let p2 = red_tiles[j];

            let width = p1.x.max(p2.x) - p1.x.min(p2.x) + 1;
            let height = p1.y.max(p2.y) - p1.y.min(p2.y) + 1;
            let potential_area = width * height;

            // Пропускаем, если потенциальная площадь меньше текущего максимума
            if potential_area <= max_area {
                continue;
            }

            if check_rectangle_optimized(p1, p2, &rows, &cols) {
                max_area = potential_area;
                println!("New max area: {} between {:?} and {:?}", max_area, p1, p2);
            }
        }
    }

    max_area
}

fn find_green_boundary(red_tiles: &[Point]) -> HashSet<Point> {
    let mut green = HashSet::new();
    let n = red_tiles.len();

    for i in 0..n {
        let p1 = red_tiles[i];
        let p2 = red_tiles[(i + 1) % n];
        add_line_points(p1, p2, &mut green);
    }

    green
}

fn add_line_points(p1: Point, p2: Point, green: &mut HashSet<Point>) {
    if p1.x == p2.x {
        let (min_y, max_y) = if p1.y < p2.y {
            (p1.y, p2.y)
        } else {
            (p2.y, p1.y)
        };
        for y in (min_y + 1)..max_y {
            green.insert(Point { x: p1.x, y });
        }
    } else if p1.y == p2.y {
        let (min_x, max_x) = if p1.x < p2.x {
            (p1.x, p2.x)
        } else {
            (p2.x, p1.x)
        };
        for x in (min_x + 1)..max_x {
            green.insert(Point { x, y: p1.y });
        }
    }
}

fn find_green_interior(
    boundary_tiles: &HashSet<Point>,
    min_x: u64,
    max_x: u64,
    min_y: u64,
    max_y: u64,
) -> HashSet<Point> {
    let mut interior = HashSet::new();
    let mut visited = HashSet::new();

    let start_x = (min_x + max_x) / 2;
    let start_y = (min_y + max_y) / 2;
    let start = Point {
        x: start_x,
        y: start_y,
    };

    if boundary_tiles.contains(&start) {
        return interior;
    }

    let mut queue = vec![start];
    visited.insert(start);

    while let Some(point) = queue.pop() {
        interior.insert(point);

        let neighbors = [
            Point {
                x: point.x.wrapping_sub(1),
                y: point.y,
            },
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y.wrapping_sub(1),
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
        ];

        for neighbor in &neighbors {
            if neighbor.x < min_x || neighbor.x > max_x || neighbor.y < min_y || neighbor.y > max_y
            {
                continue;
            }

            if visited.contains(neighbor) || boundary_tiles.contains(neighbor) {
                continue;
            }

            visited.insert(*neighbor);
            queue.push(*neighbor);
        }
    }

    interior
}

// Оптимизированная проверка: проверяем только границы прямоугольника
fn check_rectangle_optimized(
    p1: Point,
    p2: Point,
    rows: &std::collections::HashMap<u64, BTreeSet<u64>>,
    cols: &std::collections::HashMap<u64, BTreeSet<u64>>,
) -> bool {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    // Проверяем верхнюю и нижнюю строки: все x от min_x до max_x должны быть в этих строках
    if let Some(row_min_y) = rows.get(&min_y) {
        // Проверяем, что все x от min_x до max_x присутствуют
        let range = row_min_y.range(min_x..=max_x).count();
        if range != (max_x - min_x + 1) as usize {
            return false;
        }
    } else {
        return false;
    }

    if let Some(row_max_y) = rows.get(&max_y) {
        let range = row_max_y.range(min_x..=max_x).count();
        if range != (max_x - min_x + 1) as usize {
            return false;
        }
    } else {
        return false;
    }

    // Проверяем левый и правый столбцы: все y от min_y до max_y должны быть в этих столбцах
    if let Some(col_min_x) = cols.get(&min_x) {
        let range = col_min_x.range(min_y..=max_y).count();
        if range != (max_y - min_y + 1) as usize {
            return false;
        }
    } else {
        return false;
    }

    if let Some(col_max_x) = cols.get(&max_x) {
        let range = col_max_x.range(min_y..=max_y).count();
        if range != (max_y - min_y + 1) as usize {
            return false;
        }
    } else {
        return false;
    }

    true
}
