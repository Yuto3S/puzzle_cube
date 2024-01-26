use std::{collections::{HashMap, HashSet}, time::SystemTime};

pub fn find_solution_from_overlaps(overlaps: &HashMap<usize, Vec<usize>>) -> bool {
    for key in 0..overlaps.len() {
        let mut current_solution: [usize; 25] = [0; 25];
        // let mut used_overlaps: Vec<usize> = vec![];
        let mut used_overlaps: HashSet<usize> = HashSet::new();
        used_overlaps.extend(&overlaps[&key]);
        
        if find_solution_from_overlaps_recursive(
            &mut current_solution,
            1,
            key,
            &mut used_overlaps,
            &overlaps,
        ) {
            return true;
        }
    }

    return false;
}

fn find_solution_from_overlaps_recursive(
    current_solution: &mut [usize; 25],
    current_index: usize,
    current_key: usize,
    used_overlaps: &mut HashSet<usize>,
    all_overlaps: &HashMap<usize, Vec<usize>>,
) -> bool {
    if current_index < 14 {
        println!(
            "index:({}) - {:?} - solution: {:?}",
            current_index, SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(), current_solution
        );
    }

    if current_index == 25 {
        println!("{:?}", current_solution);
        return true;
    }

    for key in (current_key + 1)..all_overlaps.len() {
        if !used_overlaps.contains(&key) {
            current_solution[current_index] = key;

            let mut new_used_overlaps: HashSet<usize> = used_overlaps.clone();
            new_used_overlaps.extend(&all_overlaps[&key]);

            // used_overlaps.extend(&all_overlaps[&key]);

            if find_solution_from_overlaps_recursive(
                current_solution,
                current_index + 1,
                key,
                &mut new_used_overlaps,
                all_overlaps,
            ) {
                return true;
            }
            // current_solution[current_index] = 0;
            // used_overlaps.truncate(used_overlaps.len() - all_overlaps[&key].len())
        }
    }

    return false;
}