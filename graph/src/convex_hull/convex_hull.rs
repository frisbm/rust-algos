use std::cmp::Ordering;

fn angle_between_points(p0: (usize, usize), p_1: (usize, usize)) -> f64 {
    let x = p_1.0 as f64 - p0.0 as f64;
    let y = p_1.1 as f64 - p0.1 as f64;
    y.atan2(x)
}

fn distance_between_points(p0: (usize, usize), p_1: (usize, usize)) -> f64 {
    let x = p_1.0 as f64 - p0.0 as f64;
    let y = p_1.1 as f64 - p0.1 as f64;
    (x.powi(2) + y.powi(2)).sqrt()
}

fn ccw(p0: (usize, usize), p_1: (usize, usize), p_2: (usize, usize)) -> f64 {
    let (x_0, y_0) = (p0.0 as f64, p0.1 as f64);
    let (x_1, y_1) = (p_1.0 as f64, p_1.1 as f64);
    let (x_2, y_2) = (p_2.0 as f64, p_2.1 as f64);
    (x_1 - x_0) * (y_2 - y_0) - (y_1 - y_0) * (x_2 - x_0)
}

pub fn convex_hull(points: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if points.len() < 3 {
        return points.clone();
    }
    let mut sorted_points = points.to_vec();
    sorted_points.sort_unstable();
    let p0 = sorted_points[0];

    sorted_points.sort_by(|&p1, &p2| {
        let angle_p1 = angle_between_points(p0, p1);
        let angle_p2 = angle_between_points(p0, p2);
        let angle_cmp = angle_p1.partial_cmp(&angle_p2).unwrap_or(Ordering::Equal);

        if angle_cmp != Ordering::Equal {
            return angle_cmp;
        }

        let distance_p1 = distance_between_points(p0, p1);
        let distance_p2 = distance_between_points(p0, p2);
        distance_p1
            .partial_cmp(&distance_p2)
            .unwrap_or(Ordering::Equal)
    });

    let mut unique_points = Vec::with_capacity(sorted_points.len());
    unique_points.push(sorted_points[0]);

    for &point in sorted_points.iter().skip(1) {
        let last_angle = angle_between_points(p0, *unique_points.last().unwrap());
        let current_angle = angle_between_points(p0, point);

        if (last_angle - current_angle).abs() > f64::EPSILON {
            unique_points.push(point);
        } else {
            // Replace with the farthest point if the angle is the same
            let last_distance = distance_between_points(p0, *unique_points.last().unwrap());
            let current_distance = distance_between_points(p0, point);

            if current_distance > last_distance {
                *unique_points.last_mut().unwrap() = point;
            }
        }
    }

    let mut stack = Vec::with_capacity(points.len());
    stack.push(p0);

    for point in unique_points.iter() {
        while stack.len() > 1 && ccw(stack[stack.len() - 2], stack[stack.len() - 1], *point) <= 0.0
        {
            stack.pop();
        }
        stack.push(*point);
    }

    stack.shrink_to_fit();
    stack
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::grid::{clean_test_input, str_to_char_grid, str_to_char_map};

    const SMALL_INPUT: &str = "
        ####..####
        #........#
        ###....###
        #........#
        ####..####
    ";

    const SMALL_INPUT_EXPECTED: &str = "
        ####..####
        #.######.#
        ##########
        #.######.#
        ####..####
    ";

    #[test]
    fn test_convex_hull_small() {
        let contents = clean_test_input(SMALL_INPUT);
        let test_input = str_to_char_grid(contents.as_str());
        let input = str_to_char_map(contents.as_str());
        let filtered = input
            .iter()
            .filter(|(_, &v)| v != '#')
            .map(|(k, _)| *k)
            .collect::<Vec<(usize, usize)>>();
        let result = convex_hull(&filtered);
        let result_grid = res_to_grid(&result, test_input[0].len(), test_input.len());

        let expected = clean_test_input(SMALL_INPUT_EXPECTED);
        let expected = str_to_char_grid(expected.as_str());
        assert_eq!(expected, result_grid);
        assert_eq!(
            vec![
                (0, 4),
                (1, 1),
                (3, 1),
                (4, 4),
                (4, 5),
                (3, 8),
                (1, 8),
                (0, 5)
            ],
            result
        );
    }

    const LARGE_INPUT: &str = "
        ##############################
        ##############..##############
        ###########........###########
        ####......................####
        #####....................#####
        #............................#
        ####......................####
        #............................#
        #########..##....##..#########
        #####..##.....##.....##..#####
        ############..##..############
    ";

    const LARGE_INPUT_EXPECTED: &str = "
        ##############################
        ##############..##############
        ##############################
        ####.####################.####
        ##############################
        #.##########################.#
        ##############################
        #.##########################.#
        ##############################
        #####.##################.#####
        ############.####.############
    ";

    #[test]
    fn test_convex_hull_large() {
        let contents = clean_test_input(LARGE_INPUT);
        let test_input = str_to_char_grid(contents.as_str());
        let input = str_to_char_map(contents.as_str());
        let filtered = input
            .iter()
            .filter(|(_, &v)| v != '#')
            .map(|(k, _)| *k)
            .collect::<Vec<(usize, usize)>>();
        let result = convex_hull(&filtered);
        let result_grid = res_to_grid(&result, test_input[0].len(), test_input.len());

        result_grid.iter().for_each(|row| {
            println!("{}", row.into_iter().collect::<String>());
        });

        let expected = clean_test_input(LARGE_INPUT_EXPECTED);
        let expected = str_to_char_grid(expected.as_str());
        assert_eq!(expected, result_grid);
        assert_eq!(
            vec![
                (1, 14),
                (3, 4),
                (5, 1),
                (7, 1),
                (9, 5),
                (10, 12),
                (10, 17),
                (9, 24),
                (7, 28),
                (5, 28),
                (3, 25),
                (1, 15)
            ],
            result
        );
    }

    fn res_to_grid(res: &Vec<(usize, usize)>, width: usize, height: usize) -> Vec<Vec<char>> {
        let mut res = res.to_owned();
        res.sort();
        let mut chars: Vec<Vec<char>> = (0..height)
            .map(|_| (0..width).map(|_| '#').collect())
            .collect();
        for (x, y) in res {
            chars[x][y] = '.';
        }
        chars
    }
}
