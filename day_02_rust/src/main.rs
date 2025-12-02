fn main(){
    let input = include_str!("./input.txt");

    let invalid_id_sum =
        input
        .split(',')
        .flat_map(|range| part1(range))
        .sum::<u64>();

    let invalid_id_sum2 =
        input
        .split(',')
        .flat_map(|range| part2(range))
        .sum::<u64>();

    println!("The first sum of invalid IDs is {}", invalid_id_sum);
    println!("The second sum of invalid IDs is {}", invalid_id_sum2);
}

/*
The ranges are separated by commas (,)
each range gives its first ID and last ID separated by a dash (-).

Since the young Elf was just doing silly patterns, you can find the invalid IDs by
looking for any ID which is made only of some sequence of digits repeated twice.
So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.

None of the numbers have leading zeroes; 0101 isn't an ID at all.
(101 is a valid ID that you would ignore.)

Your job is to find all of the invalid IDs that appear in the given ranges.
*/

fn part1(range: &str) -> Vec<u64> {
    range
        .split('-')
        .collect::<Vec<_>>()
        .try_into()
        .map(|[start, stop]: [&str; 2]| {
            start.parse::<u64>().unwrap()..=stop.parse::<u64>().unwrap()
        })
        .expect("invalid range")
        .filter(|id| {
            let id_str = id.to_string();
            
            if id_str.len() % 2 != 0 {
                return false};
            
            let (a, b) = id_str.split_at(id_str.len() / 2);
            if a == b {
                true
            }
            else {
                false
            }
        })
        .collect()
}

/*
Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice.
So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times),
and 1111111 (1 seven times) are all invalid IDs.
 */

 fn part2(range: &str) -> Vec<u64> {

    range
        .split('-')
        .collect::<Vec<_>>()
        .try_into()
        .map(|[start, stop]: [&str; 2]| {
            start.parse::<u64>().unwrap()..=stop.parse::<u64>().unwrap()
        })
        .expect("invalid range")
        .filter(|id| {
            let id_str = id.to_string();
            let id_str_len = id_str.len();
            let max_chunk_size = id_str.len().div_euclid(2);

            (1..=max_chunk_size).any(|chunk_size| {
                if id_str_len % chunk_size != 0 {
                    return false
                }

                id_str
                    .as_bytes()
                    .chunks_exact(chunk_size)
                    .collect::<Vec<_>>()
                    .windows(2)
                    .all(|w| w[0] == w[1])
            })
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
/*
11-22 has two invalid IDs, 11 and 22.
95-115 has one invalid ID, 99.
998-1012 has one invalid ID, 1010.
1188511880-1188511890 has one invalid ID, 1188511885.
222220-222224 has one invalid ID, 222222.
1698522-1698528 contains no invalid IDs.
446443-446449 has one invalid ID, 446446.
38593856-38593862 has one invalid ID, 38593859.
The rest of the ranges contain no invalid IDs.
565653-565659, 824824821-824824827,2121212118-2121212124
*/

    #[test]
    fn example01() {
        let result = part1("11-22").len();
        assert_eq!(result, 2);
    }

    #[test]
    fn example02() {
        let result = part1("95-115").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example03() {
        let result = part1("998-1012").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example04() {
        let result = part1("1188511880-1188511890").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example05() {
        let result = part1("222220-222224").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example06() {
        let result = part1("1698522-1698528").len();
        assert_eq!(result, 0);
    }

    #[test]
    fn example07() {
        let result = part1("446443-446449").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example08() {
        let result = part1("38593856-38593862").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example09() {
        let result = part1("565653-565659").len();
        assert_eq!(result, 0);
    }

    #[test]
    fn example10() {
        let result = part1("824824821-824824827").len();
        assert_eq!(result, 0);
    }

    #[test]
    fn example11() {
        let result = part1("2121212118-2121212124").len();
        assert_eq!(result, 0);
    }

/*
Adding up all the invalid IDs in this example produces 1227775554.
*/

#[test]
    fn example12() {
        let example = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = example
            .split(',')
            .flat_map(|range| part1(range))
            .sum::<u64>();
        assert_eq!(result, 01227775554);
    }

    #[test]
    fn example13() {
        let result = part2("11-22").len();
        assert_eq!(result, 2);
    }

    #[test]
    fn example14() {
        let result = part2("95-115").len();
        assert_eq!(result, 2);
    }

    #[test]
    fn example15() {
        let result = part2("998-1012").len();
        assert_eq!(result, 2);
    }

    #[test]
    fn example16() {
        let result = part2("1188511880-1188511890").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example17() {
        let result = part2("222220-222224").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example18() {
        let result = part2("1698522-1698528").len();
        assert_eq!(result, 0);
    }

    #[test]
    fn example19() {
        let result = part2("446443-446449").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example20() {
        let result = part2("38593856-38593862").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example21() {
        let result = part2("565653-565659").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example22() {
        let result = part2("824824821-824824827").len();
        assert_eq!(result, 1);
    }

    #[test]
    fn example23() {
        let result = part2("2121212118-2121212124").len();
        assert_eq!(result, 1);
    }

/*
Adding up all the invalid IDs in this example produces 1227775554.
*/

#[test]
    fn example24() {
        let example = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = example
            .split(',')
            .flat_map(|range| part2(range))
            .sum::<u64>();
        assert_eq!(result, 4174379265);
    }
}