use itertools::Itertools;

fn power_set<T: Clone>(items: Vec<T>) -> Vec<Vec<T>> {
   (0..=items.len())
   .map(|count| items.clone().into_iter().combinations(count))
   .flatten()
   .collect()
}


fn main() {
    let members = vec![2, 6, 8, 10,];
    let res = power_set(members);
    println!("{:?}", res);
}
