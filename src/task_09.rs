use std::cmp::Ordering;
use std::fs;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct Point{x: u64, y: u64};

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_09_sample.txt";
    let file = fs::File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut tiles: Vec<(Point)> = Vec::new();

    for line in lines {
        let coord = line?
            .trim()
            .split(',')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        tiles.push(Point{ x: coord[0], y: coord[1]});
    }

    let largest_area = solve(&tiles);
    println!("the largest area of any rectangle: {largest_area} ");

    Ok(())
}

fn solve(tiles: &[Point]) -> u64 {
    0u64
}

// Векторное произведение для определения направления поворота вектора last_point-next_point
// относительно вектора last_but_one_point-last_point
// Если > 0: левый поворот (против часовой)
// Если < 0: правый поворот (по часовой)
// Если = 0: коллинеарны (на одной прямой)
fn cross(last_but_one_point: &Point, last_point: &Point, next_point: &Point) -> i64 {
    (last_point.x as i64 - last_but_one_point.x as i64)
        * (next_point.y as i64 - last_but_one_point.y as i64)
        - (last_point.y as i64 - last_but_one_point.y as i64)
            * (next_point.x as i64 - last_but_one_point.x as i64)
}

fn convex_hull(mut tiles: Vec<Point>) -> Vec<Point> {
    let tiles_count = tiles.len();
    if tiles_count <= 1 {
        return tiles;
    }

    // опорная точка (самая нижняя, самая левая)
    let pivot_idx = tiles
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.y.cmp(&b.y).then(a.x.cmp(&b.x)))
        .unwrap()
        .0;

    tiles.swap(0, pivot_idx);
    let pivot = tiles[0];

    // сортируем остальные точки по углу относительно pivot
    // Используем cross product для сравнения, чтобы не считать углы явно
    tiles[1..].sort_by(|a, b| {
        let cp = cross(&pivot, a, b);
        if cp != 0 {
            cp.cmp(&0) // Если поворот влево, a идет раньше b
        } else {
            // Если на одной прямой, ближе к pivot идет первая
            let dist_a = (a.x - pivot.x).pow(2) + (a.y - pivot.y).pow(2);
            let dist_b = (b.x - pivot.x).pow(2) + (b.y - pivot.y).pow(2);
            dist_a.cmp(&dist_b)
        }
    });

    // строим оболочку с помощью стека
    let mut hull = Vec::new();
    for p in &tiles {
        // Пока есть хотя бы 2 точки и последние три образуют правый поворот
        while hull.len() >= 2 && cross(&hull[hull.len() - 2], &hull[hull.len() - 1], p) <= 0 {
            hull.pop();
        }
        hull.push(*p);
    }

    hull
}

fn find_extreme_points(hull: &[Point]) -> (Point, Point, Point, Point) {
    // Инициализируем экстремумы первой точкой оболочки
    let mut min_x = hull[0];
    let mut max_x = hull[0];
    let mut min_y = hull[0];
    let mut max_y = hull[0];

    for &p in &hull[1..] {
        if p.x < min_x.x { min_x = p; }
        if p.x > max_x.x { max_x = p; }
        if p.y < min_y.y { min_y = p; }
        if p.y > max_y.y { max_y = p; }
    }

    (min_x, max_x, min_y, max_y)
}
