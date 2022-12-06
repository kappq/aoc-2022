use itertools::Itertools;

fn main() {
    let contents = include_str!("input.txt");

    let get_ranges = |assignment: &str| {
        let (first, second) = assignment.split(',').collect_tuple().unwrap();
        let (first_start, first_end) = first
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        let (second_start, second_end) = second
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        let first_range = first_start..=first_end;
        let second_range = second_start..=second_end;

        (first_range, second_range)
    };

    let first_star = contents
        .lines()
        .filter(|assignment| {
            let (first_range, second_range) = get_ranges(assignment);

            second_range.start() >= first_range.start() && second_range.end() <= first_range.end()
                || first_range.start() >= second_range.start()
                    && first_range.end() <= second_range.end()
        })
        .count();

    let second_star = contents
        .lines()
        .filter(|assignment| {
            let (first_range, second_range) = get_ranges(assignment);

            for n in first_range {
                if second_range.contains(&n) {
                    return true;
                }
            }

            false
        })
        .count();

    println!("first star: {}", first_star);
    println!("second star: {}", second_star);
}
