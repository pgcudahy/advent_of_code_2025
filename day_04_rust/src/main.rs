fn main(){
    let input = include_str!("./input.txt");

    let result1 = part1(input);
    let result2 = part2(input);

    println!("The total movable rows in part 1 are {}", result1);
    println!("The total movable rows in part 2 are {}", result2);
}

/*
The rolls of paper (@) are arranged on a large grid;
the Elves even have a helpful diagram (your puzzle input)
indicating where everything is located.
The forklifts can only access a roll of paper if there are
fewer than four rolls of paper in the eight adjacent positions
*/

fn neighboring_rolls(paper_roll_vector: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let row_width = paper_roll_vector[0].len();
    let buffer_row = vec![0; row_width + 2];

    let buffered_paper_roll_vector: Vec<Vec<u8>> =
        std::iter::once(buffer_row.clone())
            .chain(paper_roll_vector
                .iter()
                .map(|row| {
                    let mut v = Vec::with_capacity(row.len() + 2);
                    v.push(0);
                    v.extend_from_slice(row);
                    v.push(0);
                    v
                })
            )
            .chain(std::iter::once(buffer_row))
            .collect();

    buffered_paper_roll_vector
        .windows(3)
        .map(|three_row| {
            three_row
                .iter()
                .enumerate()
                .map(|(index, row)| {
                    row
                        .windows(3)
                        .map(|triplet| {
                            let window_sum: u8 =
                                triplet
                                    .iter()
                                    .map(|x| *x)
                                    .sum::<u8>();
                                if index == 1 {
                                    window_sum.saturating_sub(1)
                                } else {
                                    window_sum
                                }
                            })
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<Vec<u8>>>()
                .iter()
                .fold(vec![0u8; row_width], |mut neighbors, row| {
                    for (index, value) in row.iter().enumerate() {
                        neighbors[index] += value;
                    }
                    neighbors
                    })
            })
            .collect()
}

fn part1(paper_roll_diagram: &str) -> usize {
    let paper_roll_vector: Vec<Vec<u8>> = paper_roll_diagram
        .lines()
        .map(|row_str| {
            row_str
                .chars()
                .map(|char| {
                    match char {
                        '.' => 0,
                        '@' => 1,
                        _ => panic!("invalid diagram character"),
                    }
                })
                .collect()
        })
        .collect();

    let neighboring_rolls_vector = neighboring_rolls(&paper_roll_vector);

    paper_roll_vector
        .iter()
        .zip(neighboring_rolls_vector)
        .map(|(row, neighbor_row)| {
            row
                .iter()
                .zip(neighbor_row)
                .filter(|(roll, neighbors)| {
                    match roll {
                        1 if *neighbors < 4 => true,
                        _ => false,
                    }
                })
                .count()
        })
        .sum()
}

fn remove_rolls(paper_roll_vector: &Vec<Vec<u8>>, prior_removed_rolls: usize) ->
    (Vec<Vec<u8>>, usize) {
    let neighboring_rolls_vector = neighboring_rolls(&paper_roll_vector);
    let mut removed_rolls = prior_removed_rolls;

    let new_paper_roll_vector: Vec<Vec<u8>> =
        paper_roll_vector
            .iter()
            .zip(neighboring_rolls_vector)
            .map(|(row, neighbor_row)| {
                row
                    .iter()
                    .zip(neighbor_row)
                    .map(|(roll, neighbors)| {
                        match roll {
                            1 if neighbors < 4 => {
                                removed_rolls += 1;
                                0},
                            _ => *roll,
                        }
                    })
                    .collect()
                })
            .collect();

    if removed_rolls > prior_removed_rolls {
        remove_rolls(&new_paper_roll_vector, removed_rolls)
    } else {
        (new_paper_roll_vector, removed_rolls)
    }
}

fn part2(paper_roll_diagram: &str) -> usize {
    let paper_roll_vector: Vec<Vec<u8>> = paper_roll_diagram
        .lines()
        .map(|row_str| {
            row_str
                .chars()
                .map(|char| {
                    match char {
                        '.' => 0,
                        '@' => 1,
                        _ => panic!("invalid diagram character"),
                    }
                })
                .collect()
        })
        .collect();

        remove_rolls(&paper_roll_vector, 0).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let result = part1(
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.");
        assert_eq!(result, 13);
    }

    #[test]
    fn example02() {
        let result = part2(
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.");
        assert_eq!(result, 43);
    }
}