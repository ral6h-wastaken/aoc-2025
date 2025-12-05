use super::*;
use std::{
    collections::{HashMap, HashSet},
    num,
};

pub fn solution(input: &str) -> u64 {
    let chunk_sizes: HashMap<u8, Vec<u8>> = HashMap::from([
        (2, vec![1]),
        (3, vec![1]),
        (4, vec![2]),
        (5, vec![1]),
        (6, vec![2, 3]),
        (7, vec![1]),
        // 12_12_12_12 and 1212_1212 are equivalent in range 1000_0000 9999_9999,
        (8, vec![4]),
        (9, vec![3]),
        (10, vec![2, 5]),
        (11, vec![1]),
        (12, vec![3, 4, 6]),
    ]);

    let ranges = parse_ranges(input);

    let mut invalids = HashSet::<u64>::new();

    for range in ranges {
        //here, since al the ranges have been normalised we can assume that
        //len(lower_bound) = len(upper_bound)
        //
        //  abc_def_ghi - ABC_DEF_GHI

        let lower = range.0;
        let upper = range.1;
        let len = lower.to_string().len() as u8;
        println!("range = {range:?}, l: {len}");

        for size in chunk_sizes.get(&len).unwrap() {
            println!("size: {size}");
            let num_chunks = len / size;

            let alfa_low = lower / 10u64.pow((size * (num_chunks - 1)) as u32); //upper chunk
            // let beta_low = (lower - alfa_low * 10u64.pow((size * (num_chunks - 1)) as u32))
            //     / 10u64.pow((size * (num_chunks - 2)) as u32);

            let alfa_high = upper / 10u64.pow((size * (num_chunks - 1)) as u32); //upper chunk
            // let beta_high = (upper - alfa_high * 10u64.pow((size * (num_chunks - 1)) as u32))
            //     / 10u64.pow((size * (num_chunks - 2)) as u32);

            println!("\ta_low: {alfa_low}"); // , b_low: {beta_low} ");
            println!("\ta_high: {alfa_high}"); // , b_high: {beta_high} ");

            let inf = if alfa_low
                .to_string()
                .repeat(num_chunks as usize)
                .parse::<u64>()
                .unwrap()
                >= lower
            {
                alfa_low
            } else {
                alfa_low + 1
            };

            let sup = if alfa_high
                .to_string()
                .repeat(num_chunks as usize)
                .parse::<u64>()
                .unwrap()
                <= upper
            {
                alfa_high
            } else {
                alfa_high - 1
            };

            println!("\tinf: {inf}, sup: {sup}");

            let coef_stg_2 = coef_stg_2(*size as u32, num_chunks as u32);
            //sup * (sup + 1) - sup * (sup - 1) = sup (sup + 1 -sup + 1) = 2*sup
            let mut partial = HashSet::<u64>::new();
            for i in inf..=sup {
                partial.insert(i * coef_stg_2);
            }

            println!("\tinvalids in range: {partial:?}");
            partial.into_iter().for_each(|e| {
                invalids.insert(e);
            });

            println!("\tinvalids: {invalids:?}");
        }
    }

    invalids.iter().sum()
}

fn coef_stg_2(size: u32, num_chunks: u32) -> u64 {
    let res = (0..num_chunks).fold(0, |part, exp| part + 10u64.pow(exp * size));
    println!("\tcoef({size}, {num_chunks}) = {res}");
    res
}
