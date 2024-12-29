use std::collections::HashMap;

pub fn str_to_char_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn str_to_char_map(input: &str) -> HashMap<(usize, usize), char> {
    let grid = str_to_char_grid(input);
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            map.insert((i, j), cell);
        }
    }
    map
}

pub fn clean_test_input(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(str::trim)
        .collect::<Vec<&str>>()
        .join("\n")
}
