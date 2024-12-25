pub fn str_to_char_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
