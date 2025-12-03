fn main(){
    let input = include_str!("./input.txt");

    let output_joltage =
        input
        .lines()
        .map(|battery_bank| part1(battery_bank))
        .sum::<usize>();

    let bigger_output_joltage =
        input
        .lines()
        .map(|battery_bank| part2(battery_bank))
        .sum::<usize>();

    println!("The first total joltage is {}", output_joltage);
    println!("The first total joltage is {}", bigger_output_joltage);

}

/*
Within each bank, you need to turn on exactly two batteries;
the joltage that the bank produces is equal to the number formed
by the digits on the batteries you've turned on
You'll need to find the largest possible joltage each bank can produce.
*/

fn part1(battery_bank: &str) -> usize {
    let battery_bank_length = battery_bank.len();

    let (max1, max2) = battery_bank
        .bytes()
        .map(|b| b - b'0')
        .enumerate()
        .fold((0, 0), |(first, second), (index, digit)| {
            if digit > first && index < battery_bank_length - 1 {
                (digit, 0)
            } else if digit > second {
                (first, digit)
            } else {
                (first, second)
            }
        });

    (max1 as usize) * 10 + (max2 as usize)
}

/*
Now, you need to make the largest joltage by turning on exactly
twelve batteries within each bank.
234234234234278
434234234278
*/

 fn part2(battery_bank: &str) -> usize {
    let battery_bank_length = battery_bank.len();

    let top_12_cells = battery_bank
        .bytes()
        .map(|b| b - b'0')
        .enumerate()
        .fold(vec![0; 12], |mut top_joltage, (index, digit)| {
            let start = (12 + index).saturating_sub(battery_bank_length);
            
            (start..12)
                .position(|i| digit > top_joltage[i])
                .map(|p| {
                    top_joltage[start + p] = digit;
                    top_joltage[(start + p + 1)..].fill(0u8);
                });

            top_joltage
        });

    top_12_cells
        .into_iter()
        .fold(0, |joltage, digit| {
            joltage * 10 + digit as usize
        })
}

#[cfg(test)]
mod tests {
    use super::*;
/*
In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
In 818181911112111, the largest joltage you can produce is 92.
*/

    #[test]
    fn example01() {
        let result = part1("987654321111111");
        assert_eq!(result, 98);
    }

    #[test]
    fn example02() {
        let result = part1("811111111111119");
        assert_eq!(result, 89);
    }

    #[test]
    fn example03() {
        let result = part1("234234234234278");
        assert_eq!(result, 78);
    }

    #[test]
    fn example04() {
        let result = part1("818181911112111");
        assert_eq!(result, 92);
    }
/*
In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.
*/
    #[test]
    fn example05() {
        let result = part2("987654321111111");
        assert_eq!(result, 987654321111);
    }

    #[test]
    fn example06() {
        let result = part2("811111111111119");
        assert_eq!(result, 811111111119);
    }

    #[test]
    fn example07() {
        let result = part2("234234234234278");
        assert_eq!(result, 434234234278);
    }

    #[test]
    fn example08() {
        let result = part2("818181911112111");
        assert_eq!(result, 888911112111);
    }  
}