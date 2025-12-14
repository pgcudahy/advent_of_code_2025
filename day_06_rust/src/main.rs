fn main(){
    let input = include_str!("./input.txt");

    let result1 = part1(&input);
    let result2 = part2(&input);

    println!("The total in part 1 is {}", result1);
    println!("The total in part 2 is {}", result2);
}

/*
Each problem's numbers are arranged vertically;
at the bottom of the problem is the symbol for the operation
that needs to be performed.
To check their work, cephalopod students are given the grand
total of adding together all of the answers to the individual problems
*/

fn part1(input: &str) -> usize {
    let columns: Vec<Vec<&str>> =
        input
            .lines()
            .fold(Vec::new(), |mut columns, line| {
                line.split_whitespace()
                    .enumerate()
                    .for_each(|(index, value)| {
                        if index >= columns.len() {
                            columns.push(Vec::new());
                        }
                        columns[index].push(value);
                    });
                columns
            });

    columns
        .iter()
        .map(|column| {
            match column.as_slice() {
                [operands @ .., "*"] => operands.iter()
                    .map(|v| v.parse::<usize>().expect("not a valid number"))
                    .product::<usize>(),
                [operands @ .., "+"] => operands.iter()
                    .map(|v| v.parse::<usize>().expect("not a valid number"))
                    .sum(),
                _ => panic!("invalid column format"),
            }
        })
        .sum()
}

/*
Cephalopod math is written right-to-left in columns.
Each number is given in its own column, with the most significant
digit at the top and the least significant digit at the bottom.
*/

pub struct Column {
    digits: Vec<char>,
    operation: Option<char>,
}

impl Column {
    fn new() -> Column {
        Column{digits: Vec::new(), operation: None}
    }
}

fn process_line(line: &str, columns: Vec<Column>) -> Vec<Column>{
    line
        .chars()
        .enumerate()
        .fold(columns, |mut cols, (index, value)| {
            while cols.len() <= index {
                cols.push(Column::new());
            }

            match value {
                '+' => cols[index].operation = Some('+'),
                '*' => cols[index].operation = Some('*'),
                _ => cols[index].digits.push(value),
            }

            cols
        })
}

fn part2(input: &str) -> usize {
    let columns = input
        .lines()
        .fold(Vec::new(), |cols, line| {
            process_line(line, cols)
        });

    columns
        .split(|column| column.digits.iter().all(|&c| c == ' '))
        .filter(|column_group| !column_group.is_empty())
        .map(|column_group| {
            let numbers: Vec<usize> = column_group
                .iter()
                .map(|column| {
                    column.digits
                        .iter()
                        .filter(|&value| *value != ' ')
                        .collect::<String>()
                        .parse::<usize>()
                        .expect("not a number")
                })
                .collect();

            match column_group.iter().find_map(|column| column.operation) {
                    Some('+') => numbers.into_iter().sum::<usize>(),
                    Some('*') => numbers.into_iter().product(),
                    None => panic!("column group without an operation"),
                    Some(_) => panic!("column group with invalid operation"),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let result = part1(
"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ");

        assert_eq!(result, 4277556);
    }

    #[test]
    fn example02() {
        let result = part2(
"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ");

        assert_eq!(result, 3263827);
    }
}