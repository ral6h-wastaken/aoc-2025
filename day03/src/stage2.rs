pub fn solution(lines: Vec<String>, digits: usize) -> u64 {
    let mut sum = 0_u64;

    for line in lines {
        let len = line.len();
        let mut max_in_line = vec![0; digits];

        //        98765_43211_11111
//
//        000000000000
        'main_loop: for (idx, c) in line.chars().enumerate() {
            let c = c.to_digit(10).expect("input promised :)");

            for j in 0..digits {
                //if 0 concurs then idx must be between 0..=3 === 0..=(15 - 12) === 0..=(15 - (12 - 0))
                //if 1 concurs then idx must be between 0..=4 === 0..=(15 - 11) === 0..(15 - (12 - 1)) (to be more precise between 1..=4 cause we also need to choose one before it but idc :) )
                //if 2 concurs then idx must be between 0..=5 === 0..=(15 - 10) === 0..(15 - (12 - 2))
                //...
                //if j concurs then idx must be between 0..(15 - (12 - j))
                //
                //in general let len = line.len(), let digits = max_in_line.len() then if j concurs
                //then idx must be between 0 and (len - digits + j)
                if idx <= (len - digits + j) {
                    //check line[idx] against max_in_line[j]
                    if  c > max_in_line[j] {
                        max_in_line[j] = c;
                        // max_in_line[i] = 0 for each i > j
                        for i in j+1..digits { max_in_line[i] = 0 }
                        continue 'main_loop;
                    }
                }
            }
        }

        let partial = max_in_line
            .into_iter()
            .enumerate()
            .fold(0, |part, (index, dig)| {
                part + 10_u64.pow((digits - index - 1) as u32) * (dig as u64)
            });

        println!("line {line} -> max joltage {partial}");

        sum += partial;
    }

    sum
}
