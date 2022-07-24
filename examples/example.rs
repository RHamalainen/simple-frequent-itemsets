use simple_frequent_itemsets::frequent_itemsets;

fn main() {
    let support_minimum = 2;
    let data = vec![
        vec![1, 3, 4],
        vec![2, 3, 5],
        vec![1, 2, 3, 5],
        vec![2, 5],
        vec![1, 3, 5],
    ];
    let result = frequent_itemsets(data, support_minimum);
    println!("{:?}", result);
}
