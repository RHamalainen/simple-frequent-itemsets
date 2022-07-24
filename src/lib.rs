use itertools::Itertools;
use std::{collections::HashMap, collections::HashSet, ops::Not};

/// Get unique items in jagged array.
fn get_unique(data: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut result = HashSet::new();
    for items in data {
        for item in items {
            result.insert(*item);
        }
    }
    result.iter().copied().collect_vec()
}

/// Get certain size combinations from items.
fn get_combinations(items: &[usize], size: usize) -> Vec<Vec<usize>> {
    items.iter().copied().combinations(size).collect_vec()
}

/// Count item sets' supports.
fn get_supports(
    data: &Vec<Vec<usize>>,
    combinations: &Vec<Vec<usize>>,
) -> HashMap<Vec<usize>, usize> {
    let mut supports: HashMap<Vec<usize>, usize> = HashMap::new();
    for data_items in data {
        for combination in combinations {
            let mut all_found = true;
            for item in combination {
                if data_items.contains(item).not() {
                    all_found = false;
                    break;
                }
            }
            if all_found {
                let combination = combination.clone();
                if supports.contains_key(&combination) {
                    let count = supports.get(&combination).unwrap();
                    supports.insert(combination, count + 1);
                } else {
                    supports.insert(combination.clone(), 1);
                }
            }
        }
    }
    supports
}

/// Remove item sets whose support is lower than minimum support.
fn prune<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone>(
    supports: &HashMap<T, usize>,
    support_minimum: usize,
) -> HashMap<T, usize> {
    let mut result = HashMap::new();
    for (item, support) in supports {
        if support_minimum <= *support {
            result.insert(item.clone(), *support);
        }
    }
    result
}

fn get_keys<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone>(
    items: &HashMap<T, usize>,
) -> Vec<T> {
    items.keys().cloned().collect_vec()
}

pub fn frequent_itemsets(
    data: Vec<Vec<usize>>,
    support_minimum: usize,
) -> HashMap<Vec<usize>, usize> {
    let mut result = HashMap::new();
    let items = get_unique(&data);
    let size = 1;
    let combinations = get_combinations(&items, size);
    let supports = get_supports(&data, &combinations);
    println!("{:?}", supports);
    let mut supports = prune(&supports, support_minimum);
    println!("{:?}", supports);
    for size in 1.. {
        let items = get_keys(&supports);
        let items = get_unique(&items);
        let combinations = get_combinations(&items, size);
        println!("{:?}", combinations);
        let mut supports = get_supports(&data, &combinations);
        println!("{:?}", supports);
        let mut supports = prune(&supports, support_minimum);
        println!("{:?}", supports);
        if supports.is_empty() {
            break;
        }
        result = supports.clone();
    }
    result
}
