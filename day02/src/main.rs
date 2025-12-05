use std::{fs::File, io::*};

mod stage1;
mod stage2;

type Range = (u64, u64);

const INPUT_FILE: &str = "./resources/day02/input.txt";
const COMMA: char = ',';
const DASH: char = '-';

fn main() {
    let mut input = String::new();
    BufReader::new(File::open(INPUT_FILE).unwrap())
        .read_line(&mut input)
        .unwrap();

    println!("stage 1 : {}", stage1::solution(input.trim()));
    println!("=================================STAGE 2=================================");
    println!("stage 2 : {}", stage2::solution(input.trim()));
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input
        .split(COMMA)
        .map(|s| {
            let (lower, upper) = s.split_once(DASH).expect("problem promised :)");
            return (lower.parse().unwrap(), upper.parse().unwrap()) as Range;
        })
        .map(|r| normalise_range(r))
        .flatten()
        .collect()
}

///Returns a vector of the normalized ranges computed from the input range, i.e.
///if input = [n, N], all the ranges [mi, Mi] where:
///
///1. n <= mi <= Mi < mi+1 <= Mi+1 <= N
///2. len(mi) = len(Mi) for each i
fn normalise_range(input_range: Range) -> Vec<Range> {
    //inefficient but idc :)
    let l_min = input_range.0.to_string().len();
    let l_max = input_range.1.to_string().len();

    if l_min == l_max {
        return if l_min > 1 { vec![input_range] } else { vec![] };
    }

    //we're promised l_min <= l_max => case l_min < l_max
    let mut result: Vec<Range> = vec![];

    if l_min > 1 {
        result.push((input_range.0, "9".repeat(l_min).parse().unwrap()))
    }
    //inefficient but still :)

    for n in l_min + 1..=l_max - 1 {
        result.push((10u64.pow((n - 1) as u32), "9".repeat(n).parse().unwrap()))
    }

    if l_max > 1 {
        result.push((10u64.pow((l_max - 1) as u32), input_range.1))
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stage_one() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ans1 = 1227775554_u64;
        assert_eq!(stage1::solution(input), ans1)
    }

    #[test]
    fn test_stage_two() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ans2 = 4174379265_u64;
        assert_eq!(stage2::solution(input), ans2)
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalise_range((1, 100)), vec!((10, 99), (100, 100)));
        assert_eq!(
            normalise_range((1, 1000)),
            vec!((10, 99), (100, 999), (1000, 1000))
        );
        assert_eq!(
            normalise_range((12, 1024)),
            vec!((12, 99), (100, 999), (1000, 1024))
        );
        assert_eq!(
            normalise_range((0, 10000)),
            vec![(10, 99), (100, 999), (1000, 9999), (10000, 10000)]
        );
        assert_eq!(
            normalise_range((5, 5000)),
            vec![(10, 99), (100, 999), (1000, 5000)]
        );
        assert_eq!(normalise_range((88, 888)), vec![(88, 99), (100, 888),]);
        assert_eq!(
            normalise_range((99, 1001)),
            vec![(99, 99), (100, 999), (1000, 1001)]
        );
        assert_eq!(
            normalise_range((10, 10000)),
            vec![(10, 99), (100, 999), (1000, 9999), (10000, 10_000)]
        );
        assert_eq!(
            normalise_range((50, 5000)),
            vec![(50, 99), (100, 999), (1000, 5000)]
        );
        assert_eq!(
            normalise_range((1, 99999)),
            vec![(10, 99), (100, 999), (1000, 9999), (10000, 99999)]
        );
        assert_eq!(normalise_range((100, 999)), vec![(100, 999)]);
        assert_eq!(normalise_range((1, 9)), vec![]);
        assert_eq!(normalise_range((15, 85)), vec![(15, 85)]);
        assert_eq!(normalise_range((2500, 7500)), vec![(2500, 7500)]);
        assert_eq!(
            normalise_range((2500, 75123432)),
            vec![
                (2500, 9999),
                (10000, 99999),
                (100000, 999999),
                (1000000, 9999999),
                (10000000, 75123432)
            ]
        );
    }
}
