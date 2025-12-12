use std::{
    cmp, collections::{BTreeMap, HashSet}, str::FromStr
};

use crate::common::Point;

pub fn solution<T>(lines: T, limit: usize) -> u64
where
    T: Iterator<Item = String>,
{
    let points: Vec<Point> = lines
        .map(|l| Point::from_str(&l).expect("input promised valid lines :("))
        .collect();
    // //println("parsed points {points:?}");

    let mut distances = BTreeMap::<u64, Vec<(Point, Point)>>::new();

    for (i, p) in points.iter().enumerate() {
        for (j, q) in points.iter().enumerate() {
            if j <= i {
                continue;
            }

            let d_squared = p.distance_squared(q);
            distances
                .entry(d_squared)
                .and_modify(|v| v.push((p.clone(), q.clone())))
                .or_insert(vec![(p.clone(), q.clone())]);
        }
    }

    //println!("computed distances {distances:?}");

    let mut circuits = Vec::<HashSet<Point>>::new();
    let mut connections = 0;

    'main_loop: for (p, q) in distances.values().flatten() {
        if connections == limit {
            break 'main_loop;
        }

        //println("current connections: {connections}, current circuits: {circuits:?}");
        //println("connecting points {p:?} {q:?}");

        let clusters = circuits
            .iter()
            .enumerate()
            .filter_map(|(i, v)| match (v.contains(p), v.contains(q)) {
                (true, true) => Some((Some(i), Some(i))),
                (true, false) => Some((Some(i), None)),
                (false, true) => Some((None, Some(i))),
                (false, false) => None,
            })
            .fold(
                (None::<usize>, None::<usize>),
                |(part_p, part_q), (cur_p, cur_q)| match ((part_p, part_q), (cur_p, cur_q)) {
                    ((None, None), (Some(i), None)) => (Some(i), None),
                    ((None, None), (None, Some(i))) => (None, Some(i)),
                    ((None, None), (Some(i), Some(j))) => (Some(i), Some(j)),

                    ((Some(j), None), (None, Some(i))) => (Some(j), Some(i)),
                    ((None, Some(j)), (Some(i), None)) => (Some(i), Some(j)),
                    _ => panic!("should never happen"),
                },
            );

        //println("clusters {clusters:?}");

        match clusters {
            (None, None) => {
                // neither p or q is present we insert a new cluster
                connections += 1;
                circuits.push(HashSet::from([p.clone(), q.clone()]));
            }
            (None, Some(i)) => { // q is in a cluster but p is not -> we put p in the same cluster as q
                connections += 1;
                circuits.get_mut(i).unwrap().insert(p.clone());
            }
            (Some(i), None) => { // p is in a cluster but q is not -> we put q in the same cluster as p
                connections += 1;
                circuits.get_mut(i).unwrap().insert(q.clone());
            }
            (Some(i), Some(j)) => {
                if i == j { //we do nothing since they're already in the same cluster
                    connections += 1;
                } else { //we join clusters i and j
                    let (max_idx, min_idx) = (cmp::max(i, j), cmp::min(i, j));
                    let cluster_max = circuits.remove(max_idx);
                    let cluster_min = circuits.remove(min_idx);

                    circuits.push(cluster_max.union(&cluster_min).map(|pt| pt.clone()).collect());
                    connections += 1;
                }
            }
        }

        //println("circuits after computations {circuits:?}");
    }

    // println!("computed circuits: {circuits:?}");
    let mut circuit_lengths = circuits.iter()
        .map(|s| s.len())
        .collect::<Vec<usize>>();

    println!("circuit lengths: {circuit_lengths:?}");

    circuit_lengths.sort();

    circuit_lengths.iter().rev().take(3).fold(1, |part, cur| part * cur) as u64
}
