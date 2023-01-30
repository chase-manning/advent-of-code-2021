use crate::utils::files::get_data_as_lines;

fn get_depths() -> Vec<u32> {
    get_data_as_lines("day_1.txt")
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn get_sum(depths: &Vec<u32>, position: &usize, amount: usize) -> u32 {
    (0..amount).map(|i| depths[position - i]).sum()
}

fn larger_depth_count(amount: usize) -> usize {
    let depths = get_depths();
    (amount..depths.len())
        .reduce(
            |acc, i| match get_sum(&depths, &i, amount) > get_sum(&depths, &(i - 1), amount) {
                true => acc + 1,
                false => acc,
            },
        )
        .unwrap()
}

pub fn part_1() -> String {
    larger_depth_count(1).to_string()
}

pub fn part_2() -> String {
    larger_depth_count(3).to_string()
}

#[test]
fn result() {
    assert_eq!(part_1(), "1713");
    assert_eq!(part_2(), "71124");
}
