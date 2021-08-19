mod insertion_sort;

fn main() {
    let mut ar = vec![15, 9, 8, 1, 4, 11, 7, 12, 13, 6, 5, 3, 16, 2, 10, 14];
    insertion_sort::sort(&mut ar);
}
