use std::fs;
use std::io::{self, BufRead, BufReader};

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_02.txt";
    let file = fs::File::open(FILE_PATH)?;
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let mut total_invalid_ids_sum = 0;
    for s in first_line.trim().split(',').filter(|s| !s.is_empty()) {
        total_invalid_ids_sum += calc_invalid_ids_sum(s).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "calc_invalid_ids не вернула результат!",
            )
        })?;
        println!("total_invalid_ids_sum {:?}", total_invalid_ids_sum);
    }

    Ok(())
}

fn calc_invalid_ids_sum(ids_range: &str) -> Option<u128> {
    println!("{:?}", ids_range);

    let (start_range, end_range) = ids_range.split_once('-')?;

    let start_range = start_range.parse::<u128>().ok()?;
    let end_range = end_range.parse::<u128>().ok()?;
    println!("start_range {:?} end_width {:?}", start_range, end_range);

    let start_width = start_range.ilog10() + 1;
    let end_width = end_range.ilog10() + 1;

    println!("start_width {:?} end_width {:?}", start_width, end_width);

    // в случае нечетного количества знаков получаем меньше на единицу
    let start_half_width = start_width / 2;
    let end_half_width = end_width / 2;

    println!(
        "start_half_width {:?} end_half_width {:?}",
        start_half_width, end_half_width
    );

    let min_pattern = if !start_width.is_multiple_of(2) {
        10_u128.pow(start_half_width as u32)
    } else {
        let left_part = start_range / 10_u128.pow(start_half_width as u32);
        let right_part = start_range % 10_u128.pow(start_half_width as u32);
        println!("left_part {:?} right_part {:?}", left_part, right_part);
        if right_part == 0 {
            left_part - 1
        } else {
            if left_part < right_part {
                left_part + 1
            } else {
                left_part
            }
        }
    };
    println!("min_pattern {:?}", min_pattern);

    let max_pattern = if !end_width.is_multiple_of(2) {
        10_u128.pow(end_half_width as u32) - 1u128
    } else {
        let left_part = end_range / 10_u128.pow(end_half_width as u32);
        let right_part = end_range % 10_u128.pow(end_half_width as u32);
        println!("left_part {:?} right_part {:?}", left_part, right_part);
        if right_part == 0 {
            left_part - 1
        } else {
            if left_part > right_part {
                left_part - 1
            } else {
                left_part
            }
        }
    };
    println!("max_pattern {:?}", max_pattern);

    //let mut invalid_ids_cnt = 0;
    let mut invalid_ids_sum = 0;

    let aligned_start_half_width = start_width.div_ceil(2);

    for i in aligned_start_half_width..=end_half_width {
        println!("i {:?}", i);
        let mx = max_pattern.min(10_u128.pow(i as u32) - 1);
        let mn = min_pattern.max(10_u128.pow(i as u32 - 1));
        println!("mn {:?} mx {:?}", mn, mx);
        //invalid_ids_cnt += 1 + mx - mn;
        let half_invalid_ids_sum = (mn + mx) * (1 + mx - mn) / 2;
        println!("half_invalid_ids_sum {:?}", half_invalid_ids_sum);
        invalid_ids_sum += half_invalid_ids_sum + half_invalid_ids_sum * 10_u128.pow(i as u32);
        println!("half_invalid_ids_sum {:?}", half_invalid_ids_sum);
    }

    Some(invalid_ids_sum)
}
