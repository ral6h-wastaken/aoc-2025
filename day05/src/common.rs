use std::{env::current_exe, ops::RangeInclusive};

pub fn update_ranges(ranges: &mut Vec<RangeInclusive<u64>>, (min, max): (u64, u64)) {
    let len = ranges.len();
    let mut to_insert = true;

    for (i, range) in ranges.iter_mut().enumerate() {
        /*
         *               [n_i                       N_i]
         *                                               -------> [min           N_i] and viceversa
         *           [min                        max]
         *
         *       [min                                max]
         *                                                   -------> [min           max] and viceversa
         *           [n_i                        N_i]
         *
         *           [min    max]
         *                                       -----> continue or insert if i = len(ranges)
         *                        [n_i    max]
         */

        let n_i = range.start();
        let N_i = range.end();

        if min <= *n_i {
            // [min max] [n_i N_i]
            if max < *n_i {
                if i == len - 1 {
                    break;
                } else {
                    continue;
                }
            // [min n_i max N_i] -> [min N_i]
            } else if max <= *N_i {
                to_insert = false;
                *range = min..=*N_i;
                break;
            // [min n_i N_i max] -> [min max]
            } else if max > *N_i {
                to_insert = false;
                *range = min..=max;
                break;
            }
        } else {
            // n_i min
            // [n_i N_i] [min max]
            if *N_i < min {
                if i == len - 1 {
                    to_insert = true;
                    break;
                } else {
                    continue;
                }
            // [n_i min N_i max] -> [n_i max]
            } else if *N_i <= max {
                to_insert = false;
                *range = *n_i..=max;
                break;
            // [n_i min max N_i] -> [n_i N_i] aka do nothing
            } else if max > *N_i {
                to_insert = false;
                break;
            }
        }
    }

    if to_insert {
        ranges.push(min..=max);
    }

    coalesce_ranges(ranges)
}

//todo: fix logic
fn coalesce_ranges(ranges: &mut Vec<RangeInclusive<u64>>) {
    'main_loop: loop {
        println!("reducing ranges {:?}", ranges);
        let len = ranges.len();

        for i in 0..len {
            for j in 1..len-i {
                let j = i + j;

                let (current_slice, next_slice) = ranges.split_at_mut(j);
                let curr = current_slice.get(i).expect(format!("cannot find index {} in slice {:?}", i, current_slice).as_str());
                let next: &mut RangeInclusive<u64> = next_slice.get_mut(0).expect("cannot find index 0 in second slice");

                if let Some(to_replace) = coalesce_single(
                    (*curr.start(), *curr.end()),
                    (*next.start(), *next.end())
                ) {
                    *next = to_replace;
                    ranges.remove(i);

                    continue 'main_loop;
                }
            }
        }

        println!("No more ranges to be reduced found in {:?}", ranges);
        break 'main_loop;
    }

    println!("reduced ranges {:?}", ranges);
}

fn coalesce_single(
    (first_min, first_max): (u64, u64),
    (second_min, second_max): (u64, u64),
) -> Option<RangeInclusive<u64>> {
    println!(
        "comparing {:?} and {:?}",
        (first_min, first_max),
        (second_min, second_max)
    );

    if first_min <= second_min {
        if first_max < second_min {
            println!("disjoint");
            None
        } else if first_max <= second_max {
            println!("returning {:?}", first_min..=second_max);
            Some(first_min..=second_max)
        } else if first_max > second_max {
            println!("returning {:?}", first_min..=first_max);
            Some(first_min..=first_max)
        } else {
            panic!("should never happen!")
        }
    } else {
        if second_max < first_min {
            println!("disjoint");
            None
        } else if second_max <= first_max {
            println!("returning {:?}", second_min..=first_max);
            Some(second_min..=first_max)
        } else if second_max > first_max {
            println!("returning {:?}", second_min..=second_max);
            Some(second_min..=second_max)
        } else {
            panic!("should never happen 2!")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::coalesce_single;

    #[test]
    fn test_coalesce_single() {
        use super::*;
        assert_eq!(coalesce_single((1, 12), (8, 14)), Some(1..=14));
        assert_eq!(coalesce_single((1, 12), (13, 14)), None);
        assert_eq!(coalesce_single((100, 112), (82, 102)), Some(82..=112));
        assert_eq!(coalesce_single((100, 112), (101, 102)), Some(100..=112));
        assert_eq!(coalesce_single((101, 102), (100, 112)), Some(100..=112));
    }
}
