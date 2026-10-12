//use std::cmp::Ordering;
use std::fs;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    x: u64,
    y: u64,
}

struct VerticalEdge {
    x: u64,
    y_min: u64,
    y_max: u64,
}

struct HorizontalEdge {
    y: u64,
    x_min: u64,
    x_max: u64,
}

fn find_edges(tiles: &[Tile]) -> (Vec<VerticalEdge>, Vec<HorizontalEdge>) {
    let mut vertical_edges: Vec<VerticalEdge> = Vec::new();
    let mut horizontal_edges: Vec<HorizontalEdge> = Vec::new();

    let current_iter = tiles.iter();
    let next_iter = tiles.iter().skip(1).chain(tiles.iter().take(1));

    for (current_tile, next_tile) in current_iter.zip(next_iter) {
        if current_tile.x == next_tile.x {
            let vertical_edge = VerticalEdge {
                x: current_tile.x,
                y_min: current_tile.y.min(next_tile.y),
                y_max: current_tile.y.max(next_tile.y),
            };
            vertical_edges.push(vertical_edge);
        } else {
            let horizontal_edge = HorizontalEdge {
                y: current_tile.y,
                x_min: current_tile.x.min(next_tile.x),
                x_max: current_tile.x.max(next_tile.x),
            };
            horizontal_edges.push(horizontal_edge);
        }
    }
    (vertical_edges, horizontal_edges)
}

// плитка внутри области?
fn tile_inside(tile: &Tile, vertical_edges: &[VerticalEdge]) -> bool {
    vertical_edges
        .iter()
        .map(|ve| {
            if tile.y >= ve.y_min && tile.y < ve.y_max && tile.x > ve.x {
                1u32
            } else {
                0u32
            }
        })
        .sum::<u32>()
        % 2
        != 0
}

// прямоугольник в пределах области
fn rect_valid(
    tile1: Tile,
    tile2: Tile,
    tiles: &[Tile],
    vertical_edges: &[VerticalEdge],
    horizontal_edges: &[HorizontalEdge],
) -> bool {
    let rect_x_min = tile1.x.min(tile2.x);
    let rect_x_max = tile1.x.max(tile2.x);
    let rect_y_min = tile1.y.min(tile2.y);
    let rect_y_max = tile1.y.max(tile2.y);

    let corners = [
        Tile {
            x: rect_x_min,
            y: rect_y_min,
        },
        Tile {
            x: rect_x_min,
            y: rect_y_max,
        },
        Tile {
            x: rect_x_max,
            y: rect_y_min,
        },
        Tile {
            x: rect_x_max,
            y: rect_y_max,
        },
    ];

    for corner in corners.into_iter() {
        if !tiles.contains(&corner) && !tile_inside(&corner, vertical_edges) {
            return false;
        }
    }

    if vertical_edges.iter().any(|v_edge| {
        v_edge.x > rect_x_min
            && v_edge.x < rect_x_max
            && v_edge.y_min < rect_y_max
            && v_edge.y_max > rect_y_min
    }) {
        return false;
    }
    if horizontal_edges.iter().any(|h_edge| {
        h_edge.y > rect_y_min
            && h_edge.y < rect_y_max
            && h_edge.x_min < rect_x_max
            && h_edge.x_max > rect_x_min
    }) {
        return false;
    }

    true
}

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_09.txt";
    let file = fs::File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut tiles: Vec<Tile> = Vec::new();

    for line in lines {
        let red_tile = line?
            .trim()
            .split(',')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        tiles.push(Tile {
            x: red_tile[0],
            y: red_tile[1],
        });
    }

    let (max_area, max_area_2) = solve(tiles);
    println!("the largest area of any rectangle: {max_area} ");
    println!("the largest area of inner rectangle: {max_area_2} ");

    Ok(())
}

fn solve(tiles: Vec<Tile>) -> (u64, u64) {
    //let hull = convex_hull(tiles);
    //println!("{hull:?} ");
    //let extreme_points = find_extreme_points(hull);
    //dbg!(&extreme_points);

    let mut max_area = 0;
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let tile1 = tiles[i];
            let tile2 = tiles[j];

            let width = tile1.x.max(tile2.x) - tile1.x.min(tile2.x) + 1;
            let height = tile1.y.max(tile2.y) - tile1.y.min(tile2.y) + 1;
            //max_area = max_area.max(dbg!(width * height));
            max_area = max_area.max(width * height);
        }
    }

    let (vertical_edges, horizontal_edges) = find_edges(&tiles);

    let mut max_area_2 = 0;
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let tile1 = tiles[i];
            let tile2 = tiles[j];
            if rect_valid(tile1, tile2, &tiles, &vertical_edges, &horizontal_edges) {
                let width = tile1.x.max(tile2.x) - tile1.x.min(tile2.x) + 1;
                let height = tile1.y.max(tile2.y) - tile1.y.min(tile2.y) + 1;
                //max_area = max_area.max(dbg!(width * height));
                if width * height > max_area_2 {
                    println!("{max_area_2:?} ");
                }
                max_area_2 = max_area_2.max(width * height);
            }
        }
    }

    (max_area, max_area_2)
}

/*
// векторное произведение для определения направления поворота
// вектора AB (last_point-next_point) относительно вектора OA (last_but_one_point-last_point)
// Если > 0: левый поворот (против часовой)
// Если < 0: правый поворот (по часовой)
// Если = 0: коллинеарны (на одной прямой)
fn cross(last_but_one_point: &Tile, last_point: &Tile, next_point: &Tile) -> i64 {
    (last_point.x as i64 - last_but_one_point.x as i64)
        * (next_point.y as i64 - last_but_one_point.y as i64)
        - (last_point.y as i64 - last_but_one_point.y as i64)
            * (next_point.x as i64 - last_but_one_point.x as i64)
}

fn dist_sq(a: &Tile, b: &Tile) -> u64 {
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2)
}

fn convex_hull(mut tiles: Vec<Tile>) -> Vec<Tile> {
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
fn find_extreme_points(hull: Vec<Tile>) -> Vec<Tile> {
    // инициализируем первой точкой оболочки

    let min_x = hull.iter().map(|p| p.x).min().unwrap();
    let max_x = hull.iter().map(|p| p.x).max().unwrap();
    let min_y = hull.iter().map(|p| p.y).min().unwrap();
    let max_y = hull.iter().map(|p| p.y).max().unwrap();

    hull.into_iter()
        .filter(|&p| p.x == min_x || p.x == max_x || p.y == min_y || p.y == max_y)
        .collect()
}
*/
