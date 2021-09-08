mod heap_sort;
mod insertion_sort;

fn main() {
    println!("========Insertion sort========");
    let mut ins_sort_array = vec![15, 9, 8, 1, 4, 11, 7, 12, 13, 6, 5, 3, 16, 2, 10, 14];
    println!("Initial array:");
    println!("{:?}", ins_sort_array);
    println!("Sorting process:");
    insertion_sort::sort(&mut ins_sort_array);
    println!("================");

    println!("========Heap sort========");
    let mut heap_sort_array = vec![15, 9, 8, 1, 4, 11, 7, 17, 12, 13, 6, 5, 3, 16, 2, 10, 14];
    println!("Initial array:");
    println!("{:?}", heap_sort_array);
    println!("Sorting process:");
    heap_sort::sort(&mut heap_sort_array);
    println!("================");
}
