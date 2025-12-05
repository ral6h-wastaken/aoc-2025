pub fn solution(lines: Vec<String>) -> u32 {
    let mut sum = 0;

    for line in lines {
        let mut tens = 0;
        let mut units = 0;

        for (idx, c) in line[0..line.len()].chars().enumerate() {
            let c = c.to_digit(10).expect("input promised :)");

            if idx == line.len() - 1{
                if c > units {
                    units = c
                }
                println!("index {idx}, tens {tens}, units {units}");
                continue;
            }

            if c > tens {
                tens = c;
                units = 0;
            } else if c > units {
                units = c
            }

            println!("index {idx}, tens {tens}, units {units}");
        }

        let partial = tens * 10 + units;
        println!("line {line} -> max joltage {partial}");

        sum += partial;
    }

    sum
}

