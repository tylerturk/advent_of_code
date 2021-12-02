fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
}

fn solve_part_1(contents: String) -> i32 {
    let mut first = true;
    let mut prev: i32 = 0;
    let mut increases: i32 = 0;
    for depth in contents.lines().map(|x| x.parse::<i32>().unwrap()) {
        if first {
            first = false;
            prev = depth;
            continue;
        }
        if depth > prev {
            increases += 1;
        }
        prev = depth;
    }
    increases
}

fn solve_part_2(contents: String) -> i32 {
    let mut first = true;
    let mut prev: i32 = 0;
    let mut increases: i32 = 0;
    let depths: Vec<i32> = contents.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let num_depths = depths.len();
    for (ind, depth) in depths.clone().into_iter().enumerate() {
        if ind == num_depths - 2 {
            break;
        }
        let accumulation = depth + depths.get(ind + 1).unwrap() + depths.get(ind + 2).unwrap();
        if first {
            first = false;
            prev = accumulation;
            continue;
        }
        if accumulation > prev {
            increases += 1;
        }
        prev = accumulation;
    }
    increases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 7);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 5);
    }
}