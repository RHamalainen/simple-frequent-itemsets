# simple-frequent-itemsets

Simple implementation of frequent itemsets apriori algorithm.

This crate is a work in progress.

## Example

In this example, there is only one itemset with minimum support of three.
That itemset contains numbers 2 and 3.

```rust
let dataset = vec![
    vec![1, 2, 3],
    vec![2, 3, 4],
    vec![1, 2, 3, 5],
];
let input = dataset.iter().cloned().map(ItemSet::from_iter).collect_vec();
let support_minimum = 3;
let output = frequent_itemsets(&input, support_minimum);
```

## TODO

- rename algorithm input to "transactions"
- solve what happens when minimum support is 0 or 1
- write more tests

## Applications

- market basket analysis

## Run example

`cargo run --example=example`
