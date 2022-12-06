fn main() {
    let contents = include_str!("input.txt");

    let find_distinct = |length| {
        'search: for i in 0..contents.len() - length {
            let chunk = &contents[i..(i + length)];
            for c in chunk.chars() {
                if chunk.matches(c).count() != 1 {
                    continue 'search;
                }
            }
            return i + length;
        }

        panic!("could not find string of {} distinct characters", length);
    };

    println!("first star: {}", find_distinct(4));
    println!("second star: {}", find_distinct(14));
}
