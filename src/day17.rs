use utils;
use utils::Part;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 17).unwrap();

    process_spinlock(input, part)
}

fn process_spinlock(input:String, part:Part) -> i32 {
    const INSERT_START:i32 = 2;
    let mut out = 0;

    let step_count:usize = input.trim().parse().unwrap();

    let max_insert:i32 = match part {
        Part::PartOne => 2017,
        Part::PartTwo => 50_000_000,
    };

    let mut spinlock = vec![0,1]; // This spinlock is made regardless of step_count choice
    let mut pos:usize = 1;

    let find_after = match part {
        Part::PartOne => 2017,
        Part::PartTwo => 0,
    };

    for to_insert in INSERT_START..(max_insert + 1) {
        for _ in 0..(step_count) {
            pos += 1;
            pos %= to_insert as usize;
        }
        pos += 1;

        match part {
            Part::PartOne => spinlock.insert(pos, to_insert),
            Part::PartTwo => {
                if pos == 1 { //insert is expensive, and 0 always stays at the front of the list
                    spinlock.insert(pos, to_insert);
                }
            }
        }
    }

    let mut spinlock_iter = spinlock.iter().cycle();
    spinlock_iter.find(|&&x| x == find_after).unwrap();
    let value = spinlock_iter.next().unwrap();

    out = *value;

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(process_spinlock(String::from("3"), Part::PartOne), 638)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_spinlock(String::from("3"), Part::PartTwo), 1222153)
    }
}