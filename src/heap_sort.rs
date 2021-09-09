pub fn sort(arr: &mut Vec<i32>) {
    build_heap(arr);
    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        heapify(arr, 0, i);
    }
    println!("{:?}", arr);
}

fn build_heap(arr: &mut Vec<i32>) {
    let n = arr.len();
    let i: usize = (n / 2) - 1;
    for i in (0..=i).rev() {
        heapify(arr, i, n);
    }
}

fn heapify(arr: &mut Vec<i32>, idx: usize, max: usize) {
    let mut largest = idx;
    let left = (2 * idx) + 1;
    let right = (2 * idx) + 2;

    if left < max && arr[left] > arr[idx] {
        largest = left;
    }
    if right < max && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != idx {
        arr.swap(idx, largest);
        println!("{:?}", arr);
        heapify(arr, largest, max);
    }
}
