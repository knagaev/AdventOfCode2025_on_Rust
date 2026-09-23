use std::fs;
use std::io::{self, BufRead, BufReader};

// enum для направлений
#[derive(Debug, Clone, Copy)]
enum Direction {
    Left = -1,
    Right = 1,
}

// преобразование из символа в enum
impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

fn parse_instructions(file_path: &str) -> io::Result<Vec<Instruction>> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line_result| {
            let line = line_result?;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Empty line"));
            }

            // получаем первый символ
            let first_char = trimmed
                .chars()
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Line is too short"))?;

            // преобразуем char в enum
            let direction = Direction::from_char(first_char).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unknown direction: {}", first_char),
                )
            })?;

            // парсим число
            let distance: i32 = trimmed[1..].parse().map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("Invalid number: {}", e))
            })?;

            Ok(Instruction {
                direction,
                distance,
            })
        })
        .collect()
}

pub fn local_main() {
    const DIAL_SIZE: i32 = 100;
    let mut pos: i32 = 50;

    let mut cnt_zero: u32 = 0; // число раз на ноль в конце действия
    let mut cnt_click: u32 = 0; // число раз проходов через ноль 6907

    //match parse_instructions("data/input_01_sample.txt") {
    match parse_instructions("data/input_01.txt") {
        Ok(instructions) => {
            for instruction in instructions {
                //println!("{:?}", instruction);
                let (direction, distance) = (instruction.direction as i32, instruction.distance);

                cnt_click += if direction == -1 {
                    (((distance - pos) as f32 / DIAL_SIZE as f32).floor()
                        + if pos > 0 { 1f32 } else { 0f32 }) as u32
                } else {
                    ((distance + pos) as f32 / DIAL_SIZE as f32).floor() as u32
                };

                pos += direction * instruction.distance;
                pos %= DIAL_SIZE;
                pos += if pos < 0 { DIAL_SIZE } else { 0 };

                cnt_zero += if pos == 0 { 1 } else { 0 };
                //println!("{:?} {:?}", cnt_click, pos);
            }
        }
        Err(e) => eprintln!("Ошибка: {}", e),
    }
    println!("ответ на 1: {:?}, ответ на 2:{:?}", cnt_zero, cnt_click);
}
