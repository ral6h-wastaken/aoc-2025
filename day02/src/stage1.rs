use super::*;

pub fn solution(input: &str) -> u64 {
    let ranges = parse_ranges(input);

    let mut sum = 0u64;

    for range in ranges {
        print!("range = {range:?} ");
        //here, since al the ranges have been normalised we can assume that
        //len(lower_bound) = len(upper_bound)
        let lower = range.0;
        let upper = range.1;

        let len = lower.to_string().len();
        if len % 2 == 1 {
            continue;
        }

        let k = len / 2;
        print!("k = {k} ");

        let alfa_lower = lower / 10u64.pow(k as u32);
        let beta_lower = lower - alfa_lower * 10u64.pow(k as u32);

        let alfa_upper = upper / 10u64.pow(k as u32);
        let beta_upper = upper - alfa_upper * 10u64.pow(k as u32);

        let inf: u64 = if alfa_lower >= beta_lower {
            alfa_lower
        } else {
            alfa_lower + 1
        };

        let sup: u64 = if alfa_upper <= beta_upper {
            alfa_upper
        } else {
            alfa_upper - 1
        };

        print!("inf {inf} sup {sup} ");

        let partial = coef(k as u32) * (sup * (sup + 1) - inf * (inf - 1)) / 2;

        println!("partial {partial}");

        sum += partial;
    }

    sum
}

fn coef(k: u32) -> u64 {
    let coef = 10u64.pow(k) + 1;
    print!("coef({k}) {coef} ");
    coef
}
