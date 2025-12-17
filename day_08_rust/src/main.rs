fn main(){
    let input = include_str!("./input.txt");

    let connections = 1000;
    let result1 = part1(&input, connections);
    let result2 = part2(&input);

    println!("The product of the three biggest circuits is {}", result1);
    println!("The distance is {}", result2);
}

/*
The Elves are trying to figure out which junction boxes to connect
so that electricity can reach every junction box. 
They even have a list of all of the junction boxes' positions in
3D space (your puzzle input).
Each position is given as X,Y,Z coordinates
Your list contains many junction boxes;
connect together the 1000 pairs of junction boxes which are closest together.
Afterward, what do you get if you multiply together the sizes of
the three largest circuits?
*/

use std::collections::HashSet;

fn junction_box_distance_squared(box1: &(i64, i64, i64), box2: &(i64, i64, i64)) -> usize {
    let distance = (box1.0 - box2.0) * (box1.0 - box2.0) +
    (box1.1 - box2.1) * (box1.1 - box2.1) +
    (box1.2 - box2.2) * (box1.2 - box2.2);
    distance as usize
}

fn calculate_pair_distances(junction_boxes: &Vec<(i64, i64, i64)>) -> Vec<(usize, usize, usize)> {
    let mut pair_distances: Vec<(usize, usize, usize)> =
        junction_boxes
            .iter()
            .enumerate()
            .flat_map(|(index1, box1)| {
                junction_boxes[index1 + 1..]
                    .iter()
                    .enumerate()
                    .map(move |(index2, box2)| {
                        let distance = junction_box_distance_squared(box1, box2);
                        (index1, index1 + index2 + 1, distance)
                    })
            })
            .collect();

    pair_distances.sort_unstable_by_key(|&(_, _, distance)| distance);
    pair_distances
}

fn part1(input: &str, number_connections: usize) -> usize {
    let junction_boxes: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            let coordinates: Vec<i64> = 
                line
                    .split(',')
                    .map(|number_str| number_str
                        .parse()
                        .expect("not a valid number"))
                    .collect();
            match coordinates.as_slice() {
                [x, y, z] => (*x, *y, *z),
                _ => panic!("invalid coordinates"),
            }
        })
        .collect();

    let pair_distances = calculate_pair_distances(&junction_boxes);
    
    let connected_pairs = &pair_distances[0..number_connections];

    let circuit_sets = connected_pairs
        .iter()
        .fold(Vec::new(), |circuits: Vec<HashSet<usize>>, pair| {
            let mut new_hash = HashSet::from([pair.0, pair.1]);
            
            let (hits, mut misses): (Vec<_>, Vec<_>) =
                circuits
                    .into_iter()
                    .partition(|set| !set.is_disjoint(&new_hash));

            new_hash.extend(hits.into_iter().flatten());
            misses.push(new_hash);
            misses
        });

    let mut top_three = circuit_sets
        .iter()
        .map(|set| {
            set.len()
        })
        .collect::<Vec<usize>>();

    top_three.sort_unstable_by(|a, b| b.cmp(a));

    top_three
        .into_iter()
        .take(3)
        .product()

}

fn part2(input: &str) -> usize {
    let junction_boxes: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            let coordinates: Vec<i64> = 
                line
                    .split(',')
                    .map(|number_str| number_str
                        .parse()
                        .expect("not a valid number"))
                    .collect();
            match coordinates.as_slice() {
                [x, y, z] => (*x, *y, *z),
                _ => panic!("invalid coordinates"),
            }
        })
        .collect();

    let number_junction_boxes = junction_boxes.len();
    let pair_distances = calculate_pair_distances(&junction_boxes);

    let circuit_sets = pair_distances
        .iter()
        .try_fold(Vec::new(), |circuits, pair| {
            let mut new_hash = HashSet::from([pair.0, pair.1]);
            
            let (hits, mut misses): (Vec<_>, Vec<_>) =
                circuits
                    .into_iter()
                    .partition(|set: &HashSet<_>| !set.is_disjoint(&new_hash));

            new_hash.extend(hits.into_iter().flatten());
            misses.push(new_hash);

            if misses.len() == 1 && misses.iter().map(|set: &HashSet<_>| set.len()).sum::<usize>() == number_junction_boxes {
                Err((pair.0, pair.1))
            } else {
                Ok(misses)
            }
        });

    let (x1, x2) = circuit_sets.unwrap_err();

    let x_distance = junction_boxes[x1].0 * junction_boxes[x2].0;
    x_distance as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let result = part1(
"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689", 10);

        assert_eq!(result, 40);
    }
    #[test]
    fn example02() {
        let result = part2(
"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689");

        assert_eq!(result, 25272);
    }
}

/*
pair: (
    0,
    19,
    100427,
)
pair: (
    0,
    7,
    103401,
)
pair: (
    2,
    13,
    103922,
)
pair: (
    7,
    19,
    107662,
)
pair: (
    17,
    18,
    111326,
)
pair: (
    9,
    12,
    114473,
)
pair: (
    11,
    16,
    118604,
)
pair: (
    2,
    8,
    120825,
)
pair: (
    14,
    19,
    123051,
)
pair: (
    2,
    18,
    124564,
) */