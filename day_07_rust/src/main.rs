fn main(){
    let input = include_str!("./input.txt");

    let result1 = part1(&input);
    let result2 = part2(&input);

    println!("The tachyon beam is split {} times in part 1", result1);
    println!("The total in part 2 is {}", result2);
}

/*
A tachyon beam enters the manifold at the location marked S;
tachyon beams always move downward.
Tachyon beams pass freely through empty space (.).
However, if a tachyon beam encounters a splitter (^), the beam is stopped;
instead, a new tachyon beam continues from the immediate left and
from the immediate right of the splitter.
Analyze your manifold diagram. How many times will the beam be split?
*/
use std::collections::HashSet;

fn part1(input: &str) -> usize {
    
    input
        .lines()
        .fold((0, HashSet::new()), |(splits, mut beam_positions), line| {

            let line_splits = line
                .chars()
                .enumerate()
                .map(|(index, char)| {
                    match char {
                        'S' => {
                            beam_positions.insert(index);
                            0},
                        '^' if beam_positions.contains(&index) => {
                            beam_positions.insert(index - 1);
                            beam_positions.insert(index + 1);
                            beam_positions.remove(&index);
                            1},
                        _ => 0,
                    }
                })
                .sum::<usize>();

            (splits + line_splits, beam_positions)
        })
        .0
}

/*
it's actually time itself which splits. In one timeline,
the particle went left, and in the other timeline, the particle went right.
To fix the manifold, what you really need to know is the
number of timelines active after a single particle completes
all of its possible journeys through the manifold.
*/
use std::collections::HashMap;

fn process_line(lines: &Vec<&str>, row: usize, particle_index: usize,
    cache: &mut HashMap<(usize, usize), usize>) -> usize {
    
    if let Some(&cached_result) = cache.get(&(row, particle_index)) {
        return cached_result;
    }

    if row == lines.len() {
        return 1
    }

    let result = match lines[row].chars().nth(particle_index) {
        Some('.') => {
            process_line(lines, row + 1, particle_index, cache)
        },
        Some('^') => {
            process_line(lines, row + 1, particle_index + 1, cache) +
            process_line(lines, row + 1, particle_index - 1, cache)
        },
        None => 0,
        Some(_) => {
            panic!("invalid row instruction")
        }
    };

    cache.insert((row, particle_index), result);
    result
}

fn part2(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    let start_index: usize = lines[0]
        .chars()
        .position(|c| c == 'S')
        .expect("No S in the first row");
    let mut cache = HashMap::new();

    process_line(&lines, 1, start_index, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let result = part1(
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............");

        assert_eq!(result, 21);
    }

    #[test]
    fn example02() {
        let result = part2(
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............");

        assert_eq!(result, 40);
    }
}