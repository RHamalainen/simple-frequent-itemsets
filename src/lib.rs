//! Simple implementation of frequent itemsets apriori algorithm.
//!
//! This crate is a work in progress.
//!
//! # Example
//!
//! In this example, there is only one itemset with minimum support of three.
//! That itemset contains numbers 2 and 3.
//!
//! ```rust
//! let dataset = vec![
//!     vec![1, 2, 3],
//!     vec![2, 3, 4],
//!     vec![1, 2, 3, 5],
//! ];
//! let input = dataset.iter().cloned().map(ItemSet::from_iter).collect_vec();
//! let support_minimum = 3;
//! let output = frequent_itemsets(&input, support_minimum);
//! ```
//!
//! # TODO
//!
//! - rename algorithm input to "transactions"
//! - solve what happens when minimum support is 0 or 1
//! - write more tests

use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

/// Set of unique items.
pub type ItemSet = BTreeSet<usize>;

/// Set of itemsets. Can contain duplicates.
pub type ItemSetList = Vec<ItemSet>;

/// Itemsets associated with a frequency value.
pub type Supports = HashMap<ItemSet, usize>;

/// Get unique items in itemsets.
fn solve_unique_itemset(itemsets: &ItemSetList) -> ItemSet {
    itemsets.iter().fold(ItemSet::new(), |result, itemset| {
        result.union(itemset).copied().collect()
    })
}

/// Get certain size combinations from itemset.
fn solve_combinations(itemset: &ItemSet, size: usize) -> ItemSetList {
    let mut result = ItemSetList::new();
    let combinations = itemset.iter().combinations(size).collect_vec();
    for list in combinations {
        let owned_list = list.iter().map(|&x| *x).collect_vec();
        let set = ItemSet::from_iter(owned_list);
        result.push(set);
    }
    result
}

/// Check if all combination's items are present in a data itemset.
fn combination_is_in_itemset(itemset: &ItemSet, combination: &ItemSet) -> bool {
    combination.iter().all(|item| itemset.contains(item))
}

/// Remove itemsets with support lower than minimum support.
fn prune(supports: &Supports, support_minimum: usize) -> Supports {
    supports
        .iter()
        .filter(|&(_itemset, support)| support_minimum <= *support)
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

/// Count itemsets' supports.
fn solve_supports(
    data: &ItemSetList,
    combinations: &ItemSetList,
    support_minimum: usize,
) -> Supports {
    let mut supports = Supports::new();
    for itemset in data {
        for combination in combinations {
            if combination_is_in_itemset(itemset, combination) {
                let count = supports.entry(combination.clone()).or_insert(0);
                *count += 1;
            }
        }
    }
    prune(&supports, support_minimum)
}

/// Get itemsets from supports.
fn extract_itemsets(items: &Supports) -> ItemSetList {
    ItemSetList::from_iter(items.keys().cloned().collect_vec())
}

/// Solve frequent itemsets.
#[must_use]
pub fn frequent_itemsets(data: &ItemSetList, support_minimum: usize) -> Supports {
    let mut result = Supports::new();
    let mut unique_itemset = solve_unique_itemset(data);
    for size in 1.. {
        let combinations = solve_combinations(&unique_itemset, size);
        let supports = solve_supports(data, &combinations, support_minimum);
        if supports.is_empty() {
            break;
        }
        result = supports.clone();
        let itemsets = extract_itemsets(&supports);
        unique_itemset = solve_unique_itemset(&itemsets);
    }
    result
}

/// Helper function to transform jagged array to itemset list.
#[must_use]
pub fn jagged_array_to_itemsetlist(input: &[Vec<usize>]) -> ItemSetList {
    input.iter().cloned().map(ItemSet::from_iter).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_output_with_empty_input() {
        let support_minimum = 2;
        let dataset = vec![];
        let data = jagged_array_to_itemsetlist(&dataset);
        let output = frequent_itemsets(&data, support_minimum);
        let expected = Supports::new();
        assert_eq!(output, expected);
    }

    #[test]
    fn returns_all_with_minimum_support_zero() {
        let support_minimum = 0;
        let dataset = vec![
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
        ];
        let data = jagged_array_to_itemsetlist(&dataset);
        let output = frequent_itemsets(&data, support_minimum);
        let mut expected = Supports::new();
        expected.insert(ItemSet::from_iter(vec![1]), 1);
        expected.insert(ItemSet::from_iter(vec![1, 2]), 1);
        expected.insert(ItemSet::from_iter(vec![1, 2, 3]), 1);
        expected.insert(ItemSet::from_iter(vec![1, 2, 3, 4]), 1);
        expected.insert(ItemSet::from_iter(vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(output, expected);
    }

    #[test]
    fn returns_all_with_minimum_support_one() {
        let support_minimum = 1;
        let dataset = vec![
            vec![1, 3, 4],
            vec![2, 3, 5],
            vec![1, 2, 3, 5],
            vec![2, 5],
            vec![1, 3, 5],
        ];
        let data = jagged_array_to_itemsetlist(&dataset);
        let output = frequent_itemsets(&data, support_minimum);
        let mut expected = Supports::new();
        expected.insert(ItemSet::from_iter(vec![1, 3, 4]), 1);
        expected.insert(ItemSet::from_iter(vec![2, 3, 5]), 1);
        expected.insert(ItemSet::from_iter(vec![1, 2, 3, 5]), 1);
        expected.insert(ItemSet::from_iter(vec![2, 5]), 1);
        expected.insert(ItemSet::from_iter(vec![1, 3, 5]), 1);
        assert_eq!(output, expected);
    }

    #[test]
    fn various_data_with_minimum_support_2_instance_1() {
        let support_minimum = 2;
        let dataset = vec![
            vec![1, 3, 4],
            vec![2, 3, 5],
            vec![1, 2, 3, 5],
            vec![2, 5],
            vec![1, 3, 5],
        ];
        let data = jagged_array_to_itemsetlist(&dataset);
        let output = frequent_itemsets(&data, support_minimum);
        let mut expected = Supports::new();
        expected.insert(ItemSet::from_iter(vec![1, 3, 5]), 2);
        expected.insert(ItemSet::from_iter(vec![2, 3, 5]), 2);
        assert_eq!(output, expected);
    }

    #[test]
    fn various_data_with_minimum_support_2_instance_2() {
        let support_minimum = 2;
        let dataset = vec![
            vec![1, 2, 5],
            vec![2, 4],
            vec![2, 3],
            vec![1, 2, 4],
            vec![1, 3],
            vec![2, 3],
            vec![1, 3],
            vec![1, 2, 3, 5],
            vec![1, 2, 3],
        ];
        let data = jagged_array_to_itemsetlist(&dataset);
        let output = frequent_itemsets(&data, support_minimum);
        let mut expected = Supports::new();
        expected.insert(ItemSet::from_iter(vec![1, 2, 3]), 2);
        expected.insert(ItemSet::from_iter(vec![1, 2, 5]), 2);
        assert_eq!(output, expected);
    }

    #[test]
    fn various_data_with_minimum_support_3() {
        let support_minimum = 3;
        let dataset = vec![
            vec![1, 2, 3],
            vec![2, 3, 4],
            vec![2, 3, 5],
            vec![1, 2, 3],
            vec![1, 2, 3],
        ];
        let data = jagged_array_to_itemsetlist(&dataset);
        let output = frequent_itemsets(&data, support_minimum);
        let mut expected = Supports::new();
        expected.insert(ItemSet::from_iter(vec![1, 2, 3]), 3);
        assert_eq!(output, expected);
    }
}
