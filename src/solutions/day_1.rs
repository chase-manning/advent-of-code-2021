use crate::utils::files::get_data_as_lines;

pub fn part_1() -> String {
    let lines = get_data_as_lines("day_1.txt");
    let depths = lines
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut amount_larger = 0;
    let mut last_depth = depths[0];
    for i in 1..depths.len() {
        if depths[i] > last_depth {
            amount_larger += 1;
        }
        last_depth = depths[i];
    }

    amount_larger.to_string()
}

pub fn part_2() -> String {
    let lines = get_data_as_lines("day_1.txt");

    "Meow".to_string()
}

#[test]
fn result() {
    assert_eq!(part_1(), "1713");
    // assert_eq!(part_2(), "71124");
}
