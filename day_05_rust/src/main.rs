fn main(){
    let input = include_str!("./input.txt");

    let (fresh_ingredient_set, ingredients) =
        parse_ingredients(&input);

    let result1 = part1(&fresh_ingredient_set, ingredients);
    let result2 = part2(&fresh_ingredient_set);

    println!("The total fresh ingredients in part 1 are {}", result1);
    println!("The total possilbe fresh ingredients in part 2 are {}", result2);

}

/*
The database operates on ingredient IDs.
It consists of a list of fresh ingredient ID ranges,
a blank line, and a list of available ingredient IDs.
The fresh ID ranges are inclusive: the range 3-5 means
that ingredient IDs 3, 4, and 5 are all fresh.
The ranges can also overlap
*/

fn parse_ingredients(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (fresh_str, ingredient_str) = 
        input
            .split_once("\n\n")
            .or_else(|| input.split_once("\r\n\r\n"))
            .expect("no blank line");
    
    let mut fresh_set: Vec<(usize, usize)> = fresh_str
        .lines()
        .map(|line| {
            let (start_str, stop_str) = line
                .split_once("-")
                .expect("range line must contain '-'");
            let start: usize = start_str
                .trim()
                .parse()
                .expect("invalid number");
            let stop: usize = stop_str
                .trim()
                .parse()
                .expect("invalid number");
            (start, stop)
        })
        .collect();

    fresh_set.sort_unstable();

    let mut fresh_merged_set: Vec<(usize, usize)> = Vec::new();

    for (start, end) in fresh_set {
        if let Some(last) = fresh_merged_set.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                fresh_merged_set.push((start, end));
            }
        } else {
            fresh_merged_set.push((start, end));
        }
    }

    let ingredients: Vec<usize> =
        ingredient_str
            .lines()
            .map(|line| {
                line
                    .trim()
                    .parse()
                    .expect("ingredient is not a number")
            })
            .collect();

    (fresh_merged_set, ingredients)
}

fn part1(fresh_set_ranges: &Vec<(usize, usize)>, ingredients: Vec<usize>) -> usize {
    use std::cmp::Ordering::*;

    ingredients
        .iter()
        .filter(|&ingredient| {
            fresh_set_ranges.binary_search_by(|(s, e)| {
                match ingredient {
                    ingredient if s > ingredient => Greater,
                    ingredient if e < ingredient => Less,
                    _ => Equal,
                }
            })
            .is_ok()
            })
        .count()
}

/*
How many ingredient IDs are considered to be fresh according to
the fresh ingredient ID ranges?
*/

fn part2(fresh_set_ranges: &Vec<(usize, usize)>) -> usize {
    fresh_set_ranges
        .iter()
        .map(|(start, stop)| {
            stop - start + 1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let (fresh_ingredient_set, ingredients) =
            parse_ingredients(
"3-5
10-14
16-20
12-18

1
5
8
11
17
32");

        let result = part1(&fresh_ingredient_set, ingredients);
        assert_eq!(result, 3);
    }

    #[test]
    fn example02() {
        let (fresh_ingredient_set, _ingredients) =
            parse_ingredients(
"3-5
10-14
16-20
12-18

1
5
8
11
17
32");

        let result = part2(&fresh_ingredient_set);
        assert_eq!(result, 14);
    }
}