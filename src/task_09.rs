use std::cmp::Ordering;
use std::fs;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_09.txt";
    let file = fs::File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut tiles: Vec<Point> = Vec::new();

    for line in lines {
        let coord = line?
            .trim()
            .split(',')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        tiles.push(Point {
            x: coord[0],
            y: coord[1],
        });
    }

    let largest_area = solve(tiles);
    println!("the largest area of any rectangle: {largest_area} ");

    Ok(())
}

fn solve(tiles: Vec<Point>) -> u64 {
    let hull = convex_hull(tiles);
    println!("{hull:?} ");
    //let extreme_points = find_extreme_points(hull);
    //dbg!(&extreme_points);

    let mut max_area = 0;
    for i in 0..hull.len() {
        for j in (i + 1)..hull.len() {
            let p1 = hull[i];
            let p2 = hull[j];

            let width = p1.x.max(p2.x) - p1.x.min(p2.x) + 1;
            let height = p1.y.max(p2.y) - p1.y.min(p2.y) + 1;
            max_area = max_area.max(dbg!(width * height));
            //dbg!(p1, p2, max_area);
        }
    }

    max_area
}

// векторное произведение для определения направления поворота
// вектора AB (last_point-next_point) относительно вектора OA (last_but_one_point-last_point)
// Если > 0: левый поворот (против часовой)
// Если < 0: правый поворот (по часовой)
// Если = 0: коллинеарны (на одной прямой)
fn cross(last_but_one_point: &Point, last_point: &Point, next_point: &Point) -> i64 {
    (last_point.x as i64 - last_but_one_point.x as i64)
        * (next_point.y as i64 - last_but_one_point.y as i64)
        - (last_point.y as i64 - last_but_one_point.y as i64)
            * (next_point.x as i64 - last_but_one_point.x as i64)
}

fn dist_sq(a: &Point, b: &Point) -> u64 {
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2)
}

fn convex_hull(mut tiles: Vec<Point>) -> Vec<Point> {
    let tiles_count = tiles.len();
    if tiles_count <= 1 {
        return tiles;
    }

    // опорная точка (сначала минимум по Y, потом по X - "нижняя левая")
    let pivot_idx = tiles
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.y.cmp(&b.y).then_with(|| a.x.cmp(&b.x)))
        .unwrap()
        .0;

    tiles.swap(0, pivot_idx);
    let pivot = tiles[0];

    // сортируем остальные точки по углу относительно pivot
    // Используем cross product для сравнения, чтобы не считать углы явно
    tiles[1..].sort_by(|a, b| {
        let cp = cross(&pivot, a, b);
        match cp.cmp(&0) {
            Ordering::Less => Ordering::Greater, // Правый поворот -> b идет раньше (сортируем против часовой)
            Ordering::Greater => Ordering::Less, // Левый поворот -> a идет раньше
            Ordering::Equal => {
                // Если на одной прямой, ближе к pivot идет первая
                dist_sq(&pivot, a).cmp(&dist_sq(&pivot, b))
            }
        }
    });
    //println!("tiles {tiles:?} ");

    // строим оболочку с помощью стека
    let mut hull = Vec::new();
    for p in &tiles {
        // пока есть хотя бы 2 точки и последние три образуют правый поворот
        while hull.len() >= 2 && cross(&hull[hull.len() - 2], &hull[hull.len() - 1], p) <= 0 {
            hull.pop();
            //println!("hull pop {hull:?} ");
        }
        hull.push(*p);
        //println!("hull push {hull:?} ");
    }

    hull
}

// поиск "экстремальных" (наиболее удаленных) по X и Y
fn find_extreme_points(hull: Vec<Point>) -> Vec<Point> {
    // инициализируем первой точкой оболочки

    let min_x = hull.iter().map(|p| p.x).min().unwrap();
    let max_x = hull.iter().map(|p| p.x).max().unwrap();
    let min_y = hull.iter().map(|p| p.y).min().unwrap();
    let max_y = hull.iter().map(|p| p.y).max().unwrap();

    hull.into_iter()
        .filter(|&p| p.x == min_x || p.x == max_x || p.y == min_y || p.y == max_y)
        .collect()
}
