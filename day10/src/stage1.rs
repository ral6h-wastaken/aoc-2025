use std::{collections::HashSet, fmt::format, str::FromStr};

use crate::common::{self, LightDiagram, LightState, Machine};

pub fn solution<T>(mut lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let machines = lines
        .map(|l| Machine::from_str(&l).expect(&format!("Invalid line {l}")))
        .collect::<Vec<Machine>>();

    // println!("Parsed machines {:?}", machines);

    let mut result = 0;

    for machine in machines {
        let target = machine.target();
        let buttons = machine.buttons();

        let initial_state = LightDiagram::new(vec![LightState::OFF; target.size()]);
        let mut partial_min = 0;

        let mut possible_states = HashSet::<LightDiagram>::new();
        possible_states.insert(initial_state);

        'main_loop: loop {
            partial_min += 1;
            let mut possible_states_tmp = HashSet::<LightDiagram>::new();
            for btn in buttons.iter() {
                for state in possible_states.iter() {
                    let computed = state.compute(btn);
                    // println!("Computing {state:?} and {btn:?}, result {computed:?}");
                    if computed == *target {
                        println!("Got match! computed: {computed:?}, target: {target:?}");
                        break 'main_loop;
                    }

                    possible_states_tmp.insert(computed);
                }
            }
            for tmp in possible_states_tmp {
                possible_states.insert(tmp);
            }
        }

        println!(
            "min button presses for state {:?} are {}",
            target, partial_min
        );
        result += partial_min;
    }

    result
}
