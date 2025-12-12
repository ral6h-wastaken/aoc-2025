use std::{
    cmp, collections::{BTreeMap, HashSet}, str::FromStr
};

use crate::common::Point;

pub fn solution<T>(lines: T) -> u64
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
    let mut last_connection: Option<(Point, Point)> = None;

    for (p, q) in distances.values().flatten() {
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
                circuits.push(HashSet::from([p.clone(), q.clone()]));
            }
            (None, Some(i)) => { // q is in a cluster but p is not -> we put p in the same cluster as q
                circuits.get_mut(i).unwrap().insert(p.clone());
            }
            (Some(i), None) => { // p is in a cluster but q is not -> we put q in the same cluster as p
                circuits.get_mut(i).unwrap().insert(q.clone());
            }
            (Some(i), Some(j)) => {
                if i == j { //we do nothing since they're already in the same cluster
                } else { //we join clusters i and j
                    let (max_idx, min_idx) = (cmp::max(i, j), cmp::min(i, j));
                    let cluster_max = circuits.remove(max_idx);
                    let cluster_min = circuits.remove(min_idx);

                    circuits.push(cluster_max.union(&cluster_min).map(|pt| pt.clone()).collect());
                }
            }
        }

        //println("circuits after computations {circuits:?}");
        if circuits.len() == 1 && circuits[0].len() == points.len() {
            println!("found last connection: ({p:?}, {q:?})");
            last_connection = Some((p.clone(), q.clone()));
            break;
        }
    }

    match last_connection {
        Some((p, q)) => {
            let x1 = p.raw_coordinates().0;
            let x2 = q.raw_coordinates().0;

            x1 * x2
        }
        None => panic!("Should never happen")
    }
}
