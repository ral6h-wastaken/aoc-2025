use std::str::FromStr;

use crate::common::{Operation, Operator};

pub fn solution<T>(lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let mut operations = Vec::<Operation>::new();

    for line in lines {
        let line = line.trim();
        for (i, word) in line.split_whitespace().enumerate() {
            let word = word.trim();
            if let Ok(operator) = Operator::from_str(word) {
                operations.get_mut(i).expect("we should have some operands b4 operators wtf :(").set_operator(operator);
            } else {
                let operand = word
                    .parse::<u64>()
                    .expect("input promised numbers or operators :(");

                match operations.get_mut(i) {
                    Some(operation) => operation.add_operand(operand),
                    None => {
                        let mut new_op = Operation::new();
                        new_op.add_operand(operand);
                        operations.push(new_op)
                    }
                }
            }
        }
    }

    operations
        .iter()
        .fold(0, |acc, next_op| acc + next_op.calculate().unwrap())
}

