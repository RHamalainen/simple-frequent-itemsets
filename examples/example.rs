use simple_frequent_itemsets::{frequent_itemsets, jagged_array_to_itemsetlist};

fn main() {
    let support_minimum = 2;
    let dataset = vec![
        vec![1, 3, 4],
        vec![2, 3, 5],
        vec![1, 2, 3, 5],
        vec![2, 5],
        vec![1, 3, 5],
    ];
    let input = jagged_array_to_itemsetlist(&dataset);
    let output = frequent_itemsets(&input, support_minimum);
    println!("{:?}", output);
}
