use std::path::is_separator;

use crate::common::{self, Operation, Operator};

pub fn solution<T>(lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let mut transposed = Vec::<String>::new();
    let mut operators = Vec::<Operator>::new();

    for (i, line) in lines.enumerate() {
        println!("i={i}, len={}", line.len());

        if i == 0 {
            transposed = vec!["".into(); line.len()];
        }

        println!("trans = {:?}", transposed);

        if !is_separator_line(&line) {
            for (j, c) in line.chars().enumerate() {
                println!("j={j}, c={c}");

                transposed
                    .get_mut(j)
                    .expect("should be already initialised :(")
                    .push(c);
            }
        } else {
            for s in line.split_whitespace() {
                operators.push(Operator::try_from(s).unwrap());
            }
        }
    }

    println!("transposed {:?}, operators {:?}", transposed, operators);

    let mut tmp_operands = Vec::<u64>::new();
    let mut current_idx = 0;

    let mut res = 0_u64;

    for line in transposed {
        let line: String = line.chars().filter(|c| !c.is_whitespace()).collect();

        if line.is_empty() {
            let mut operation = Operation::new();
            for op in &tmp_operands {
                operation.add_operand(*op);
            }
            operation.set_operator(operators[current_idx]);
            res += operation.calculate().unwrap();

            println!("update res {res}");

            current_idx += 1;
            tmp_operands.clear();
        } else {
            let operand = line.parse().expect("input promised :(");
            println!("pushing operand {operand}");
            tmp_operands.push(operand);
        }
    }

    let mut operation = Operation::new();
    for op in &tmp_operands {
        operation.add_operand(*op);
    }
    operation.set_operator(operators[current_idx]);
    res += operation.calculate().unwrap();

    println!("update res {res}");

    res
}

fn is_separator_line(line: &str) -> bool {
    let line = line.trim();
    for c in line.chars() {
        match Operator::try_from(c) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    false
}
