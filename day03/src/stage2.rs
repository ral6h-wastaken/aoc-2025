pub fn solution(lines: Vec<String>) -> u64 {
    let mut sum = 0_u64;

    for line in lines {
        let mut tens = 0_u64;
        let mut units = 0_u64;

        for (idx, c) in line[0..line.len()].chars().enumerate() {
            let c = c.to_digit(10).expect("input promised :)");

            if idx == line.len() - 1 {
                if c as u64> units {
                    units = c as u64
                }
                println!("index {idx}, tens {tens}, units {units}");
                continue;
            }

            if c as u64 > tens {
                tens = c as u64;
                units = 0_u64;
            } else if c as u64 > units {
                units = c as u64
            }

            println!("index {idx}, tens {tens}, units {units}");
        }

        let partial = tens * 10 + units;
        println!("line {line} -> max joltage {partial}");

        sum += partial;
    }

    sum
}

