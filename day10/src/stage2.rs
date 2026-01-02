use std::{collections::HashSet, str::FromStr};

use crate::common::Machine;

pub fn solution<T>(lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let machines = lines
        .map(|l| Machine::from_str(&l).expect(&format!("Invalid line {l}")))
        .collect::<Vec<Machine>>();

    println!("Parsed machines {:?}", machines);

    let mut result = 0;

    for machine in machines {
        let target = machine.joltage_req();
        let buttons = machine.buttons();

        let initial_joltage = vec![0; target.len()];
        let mut partial_min = 0;

        let mut possible_joltages = HashSet::<Vec<usize>>::new();
        possible_joltages.insert(initial_joltage);

        'main_loop: loop {
            partial_min += 1;
            let mut possible_jolt_tmp = HashSet::<Vec<usize>>::new();
            for btn in buttons.iter() {
                for state in possible_joltages.iter() {
                    let computed_jolt = btn.add_joltage(state);
                    // println!("Computing {state:?} and {btn:?}, result {computed_jolt:?}");
                    if computed_jolt == *target {
                        println!("Got match! computed: {computed_jolt:?}, target: {target:?}");
                        break 'main_loop;
                    }

                    possible_jolt_tmp.insert(computed_jolt);
                }
            }
            for tmp in possible_jolt_tmp {
                possible_joltages.insert(tmp);
            }
        }

        println!(
            "min button presses for joltage {:?} are {}",
            target, partial_min
        );
        result += partial_min;
    }

    result
}
