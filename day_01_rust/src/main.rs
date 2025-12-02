fn main(){
    let input = include_str!("./input.txt");

    let actual_password = 
        input
            .lines()
            .fold((50, 0),
                |(dial_position, zeros), line| part1((dial_position, zeros), line));
    
    let actual_password2 = 
        input
            .lines()
            .fold((50, 0),
                |(dial_position, zeros), line| part2((dial_position, zeros), line));

    println!("The actual password is {}", actual_password.1);
    println!("The actual *actual* password is {}", actual_password2.1);
}

/*
A rotation starts with an L or R which indicates whether the rotation should be
    to the left (toward lower numbers) or to the right (toward higher numbers)
Then, the rotation has a distance value which indicates how many clicks the dial
    should be rotated in that direction.
The dial starts by pointing at 50.

The actual password is the number of times the dial is left pointing at 0 after
    any rotation in the sequence.
*/

fn part1(position_and_zeros: (u32, u32), command: &str) -> (u32, u32) {

    let (direction, distance_str) = command.split_at(1);
    let distance = distance_str.trim()
        .parse::<u32>()
        .expect("invalid distance")
        .rem_euclid(100);
    
    let final_position: u32 = match direction {
        "L" => (position_and_zeros.0 + (100 - distance)).rem_euclid(100),
        "R" => (position_and_zeros.0 + distance).rem_euclid(100),
        _ => panic!("ruh roh"),
    };

    match final_position {
        0 => (final_position, position_and_zeros.1 + 1),
        _ => (final_position, position_and_zeros.1),
    }
}

/*
you're actually supposed to count the number of times any click causes
    the dial to point at 0, regardless of whether it happens during a
    rotation or at the end of one.
*/

fn part2(position_and_zeros: (u32, u32), command: &str) -> (u32, u32) {

    let (direction, distance_str) = command.split_at(1);
    let distance = distance_str.trim()
        .parse::<u32>()
        .expect("invalid distance");

    let final_position: u32 = match direction {
        "L" => (position_and_zeros.0 + (100 - distance.rem_euclid(100))).rem_euclid(100),
        "R" => (position_and_zeros.0 + distance).rem_euclid(100),
        _ => panic!("ruh roh"),
    };

    let passes_zero: u32 = match direction {
        "L" if distance > position_and_zeros.0 && position_and_zeros.0 == 0 =>
            (distance - position_and_zeros.0).div_euclid(100),
        "L" if distance > position_and_zeros.0 =>
            (distance - position_and_zeros.0).div_euclid(100) + 1,
        "L" if distance < position_and_zeros.0 => 0,
        "L" if distance == position_and_zeros.0 => 1,
        "R" => (position_and_zeros.0 + distance).div_euclid(100),
        _ => panic!("ruh roh"),
    };

    println!("Started at {}, turned {} to the {}, passed zero {} times, and ended at {}",
        position_and_zeros.0, distance, direction, passes_zero, final_position);
    
    (final_position, position_and_zeros.1 + passes_zero)
}

#[cfg(test)]
mod tests {
    use super::*;
/*
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
Because the dial points at 0 a total of three times during this process,
the password in this example is 3.
*/

    #[test]
    fn example01() {
        let instructions =
"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let result = 
            instructions
                .lines()
                .fold((50, 0),
                    |(dial_position, zeros), line| part1((dial_position, zeros), line));
        assert_eq!(result.1, 3);
    }

    #[test]
    fn example02() {
        let instructions =
"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let result = 
            instructions
                .lines()
                .fold((50, 0),
                    |(dial_position, zeros), line| part2((dial_position, zeros), line));
        assert_eq!(result.1, 6);
    }
}