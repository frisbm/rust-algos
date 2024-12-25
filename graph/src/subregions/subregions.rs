use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct RegionProperties {
    pub area: u64,
    pub perimeter: u64,
    pub sides: u64,
}

impl RegionProperties {
    pub fn new() -> Self {
        Self {
            area: 0,
            perimeter: 0,
            sides: 0,
        }
    }
}

fn is_same_region<T: Eq + Hash + Clone>(
    grid: &Vec<Vec<T>>,
    x: usize,
    y: usize,
    region: &T,
) -> bool {
    x < grid.len() && y < grid[0].len() && &grid[x][y] == region
}

fn get_adjacent(x: usize, y: usize) -> [(usize, usize); 4] {
    [
        (x.wrapping_sub(1), y),
        (x, y.wrapping_sub(1)),
        (x + 1, y),
        (x, y + 1),
    ]
}

fn get_diagonal(x: usize, y: usize) -> [(usize, usize); 4] {
    [
        (x.wrapping_sub(1), y.wrapping_sub(1)),
        (x + 1, y.wrapping_sub(1)),
        (x + 1, y + 1),
        (x.wrapping_sub(1), y + 1),
    ]
}

pub fn find_properties_of_subregions<T: Eq + Hash + Clone>(
    grid: Vec<Vec<T>>,
) -> HashMap<T, RegionProperties> {
    let mut properties: HashMap<T, RegionProperties> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let start = (x, y);
            if visited.contains(&start) {
                continue;
            }

            let mut region_properties = RegionProperties::new();
            queue.push_back(start);
            visited.insert(start);
            let region = grid[start.0][start.1].to_owned();
            while let Some((x, y)) = queue.pop_front() {
                region_properties.area += 1;

                let adjacent = get_adjacent(x, y);
                let diagonal = get_diagonal(x, y);

                let adjacent_same_region = adjacent
                    .iter()
                    .map(|x| is_same_region(&grid, x.0, x.1, &region))
                    .collect::<Vec<bool>>();

                let found_sides = (0..4).fold(0, |acc, i| -> u64 {
                    if !adjacent_same_region[i] && !adjacent_same_region[(i + 1) % 4] {
                        return acc + 1;
                    } else if adjacent_same_region[i]
                        && adjacent_same_region[(i + 1) % 4]
                        && !is_same_region(&grid, diagonal[i].0, diagonal[i].1, &region)
                    {
                        return acc + 1;
                    }
                    acc
                });
                region_properties.sides += found_sides;

                let found_perimeter = (0..4).into_iter().fold(0, |acc, i| -> u64 {
                    let pos = adjacent[i];
                    let is_same = adjacent_same_region[i];
                    if !is_same {
                        return acc + 1;
                    } else if !visited.contains(&pos) {
                        queue.push_back(pos);
                        visited.insert(pos);
                    }
                    acc
                });

                region_properties.perimeter += found_perimeter;
            }
            properties.insert(region, region_properties);
        }
    }
    properties
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const SMALL_INPUT: &str = "
        BBA
        AAA
        AAA
    ";

    #[test]
    fn find_properties_of_sub_regions_small() {
        let contents = SMALL_INPUT
            .trim()
            .lines()
            .map(str::trim)
            .collect::<Vec<&str>>()
            .join("\n");
        let grid = utils::grid::str_to_char_grid(contents.as_str());
        let properties = find_properties_of_subregions(grid);

        assert_eq!(properties.len(), 2);
        assert_ne!(properties.get(&'A'), None);
        assert_ne!(properties.get(&'B'), None);

        let region_a = properties.get(&'A').unwrap();
        let region_b = properties.get(&'B').unwrap();

        assert_eq!(region_a.area, 7);
        assert_eq!(region_a.perimeter, 12);
        assert_eq!(region_a.sides, 6);

        assert_eq!(region_b.area, 2);
        assert_eq!(region_b.perimeter, 6);
        assert_eq!(region_b.sides, 4);
    }

    const COMPLEX_SMALL_INPUT: &str = "
        AAAAA
        ABBBA
        ABCBA
        ABBBA
        AAAAA
    ";

    #[test]
    fn find_properties_of_sub_regions_complex_small() {
        let contents = COMPLEX_SMALL_INPUT
            .trim()
            .lines()
            .map(str::trim)
            .collect::<Vec<&str>>()
            .join("\n");
        let grid = utils::grid::str_to_char_grid(contents.as_str());
        let properties = find_properties_of_subregions(grid);

        assert_eq!(properties.len(), 3);
        assert_ne!(properties.get(&'A'), None);
        assert_ne!(properties.get(&'B'), None);
        assert_ne!(properties.get(&'C'), None);

        let region_a = properties.get(&'A').unwrap();
        let region_b = properties.get(&'B').unwrap();
        let region_c = properties.get(&'C').unwrap();

        assert_eq!(region_a.area, 16);
        assert_eq!(region_a.perimeter, 32);
        assert_eq!(region_a.sides, 8);

        assert_eq!(region_b.area, 8);
        assert_eq!(region_b.perimeter, 16);
        assert_eq!(region_b.sides, 8);

        assert_eq!(region_c.area, 1);
        assert_eq!(region_c.perimeter, 4);
        assert_eq!(region_c.sides, 4);
    }

    const LARGE_INPUT: &str = "
        AAAAAAAAAAAAAAAA
        AAAAAAAAAAAAAAAA
        AAAAAAAAAAAAAAAA
        AAAAAAAAAAAAAAAA
        AAAAAAAAAAAAAAAA
        AAAAAAAAAAAAAAAA
        AAAAAABBBBAAAAAA
        AAAAAABBBBAAAAAA
        AAAAAABBBBAAAAAA
        AAAAAABBBBAAAAAA
        AAAAAAAAAACCCAAA
        AAAAAAAAAACCCAAA
        AAAAAAAAAACCCAAA
        AAAAAAAAAAAAADDA
        AAAAAAAAAAAAADDA
        AAAAAAAAAAAAAAAE
    ";

    #[test]
    fn find_properties_of_sub_regions_large() {
        let contents = LARGE_INPUT
            .trim()
            .lines()
            .map(str::trim)
            .collect::<Vec<&str>>()
            .join("\n");
        let grid = utils::grid::str_to_char_grid(contents.as_str());
        let properties = find_properties_of_subregions(grid);

        assert_eq!(properties.len(), 5);
        assert_ne!(properties.get(&'A'), None);
        assert_ne!(properties.get(&'B'), None);
        assert_ne!(properties.get(&'C'), None);
        assert_ne!(properties.get(&'D'), None);
        assert_ne!(properties.get(&'E'), None);

        let region_a = properties.get(&'A').unwrap();
        let region_b = properties.get(&'B').unwrap();
        let region_c = properties.get(&'C').unwrap();
        let region_d = properties.get(&'D').unwrap();
        let region_e = properties.get(&'E').unwrap();

        assert_eq!(region_a.area, 226);
        assert_eq!(region_a.perimeter, 100);
        assert_eq!(region_a.sides, 18);

        assert_eq!(region_b.area, 16);
        assert_eq!(region_b.perimeter, 16);
        assert_eq!(region_b.sides, 4);

        assert_eq!(region_c.area, 9);
        assert_eq!(region_c.perimeter, 12);
        assert_eq!(region_c.sides, 4);

        assert_eq!(region_d.area, 4);
        assert_eq!(region_d.perimeter, 8);
        assert_eq!(region_d.sides, 4);

        assert_eq!(region_e.area, 1);
        assert_eq!(region_e.perimeter, 4);
        assert_eq!(region_e.sides, 4);
    }
}
