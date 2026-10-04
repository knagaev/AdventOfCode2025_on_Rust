use std::fs;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
struct JBox(u64, u64, u64);

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        // Итеративный поиск с path compression
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }

        let mut current = x;
        while current != root {
            let next = self.parent[current];
            self.parent[current] = root;
            current = next;
        }

        root
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        // union by size
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        self.components -= 1;
        true
    }

    /*
    fn get_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
    */

    fn get_all_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for i in 0..n {
            let root = self.find(i);
            if !seen.contains(&root) {
                seen.insert(root);
                sizes.push(self.size[root]);
            }
        }

        sizes
    }

    fn is_all_connected(&self) -> bool {
        self.components == 1
    }
}

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_08.txt";
    let file = fs::File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut list_jboxes: Vec<JBox> = Vec::new();

    for line in lines {
        let coord = line?
            .trim()
            .split(',')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let jbox = JBox(coord[0], coord[1], coord[2]);
        list_jboxes.push(jbox);
    }

    let (mult_three_largest, mult_last_x) = solve(&list_jboxes);
    println!(
        "Multiplication of the three largest circuits sizes: {} ",
        mult_three_largest
    );
    println!(
        "Multiplication of the X coordinates of the last two junction boxes: {} ",
        mult_last_x
    );

    Ok(())
}

fn solve(list_jboxes: &[JBox]) -> (u64, u64) {
    let jbox_count = list_jboxes.len();
    let mut distances: Vec<((usize, usize), u64)> = Vec::new();
    // расчет расстояния от i до j
    for i in 0..jbox_count - 1 {
        for j in i + 1..jbox_count {
            distances.push(((i, j), sq_dist(&list_jboxes[i], &list_jboxes[j])));
        }
    }

    distances.sort_by_key(|&(_, dist)| dist);

    let mut dsu = DSU::new(jbox_count);

    // 1000 соединений
    // let connections_to_make = 10.min(distances.len()); // для sample
    let connections_to_make = 1000.min(distances.len());
    let mut connections: usize = 0;
    let mut mult_three_largest = 0u64;
    let mut mult_last_x = 0u64;

    for &((i, j), _) in &distances {
        dsu.union(i, j);

        if connections < connections_to_make {
            connections += 1;
        } else if mult_three_largest == 0 {
            // размеры всех circuit'ов
            let mut circuit_sizes: Vec<usize> = dsu.get_all_sizes();

            // сортируем по убыванию и берём три largest
            circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));

            mult_three_largest = if circuit_sizes.len() >= 3 {
                circuit_sizes[0] as u64 * circuit_sizes[1] as u64 * circuit_sizes[2] as u64
            } else {
                0u64
            };
        }

        if dsu.is_all_connected() {
            mult_last_x = list_jboxes[i].0 * list_jboxes[j].0;
            break;
        }
    }

    (mult_three_largest, mult_last_x)
}

fn sq_dist(jb1: &JBox, jb2: &JBox) -> u64 {
    let dx = jb2.0 as i64 - jb1.0 as i64;
    let dy = jb2.1 as i64 - jb1.1 as i64;
    let dz = jb2.2 as i64 - jb1.2 as i64;

    (dx.pow(2) + dy.pow(2) + dz.pow(2)) as u64
}
