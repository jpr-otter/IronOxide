fn sort_nd_array<T: Ord>(arr: &mut [T]) {
    arr.sort();
}

fn main() {
    let mut arr = vec![vec![3, 5, 1], vec![6, 300, 4], vec![405, 1001, 404]];
    for inner_arr in arr.iter_mut() {
        sort_nd_array(inner_arr);
    }
    println!("{:?}", arr);
}
