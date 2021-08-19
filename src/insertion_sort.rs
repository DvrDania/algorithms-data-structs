pub fn sort(array: &mut Vec<i32>) {
    for i in 1..array.len() {
        insert(array, i, array[i]);
        println!("{:?}", array);
    }
}

fn insert(array: &mut Vec<i32>, pos: usize, value: i32) {
    let mut i = pos - 1;
    while array[i] > value {
        let buffer = array[i];
        array[i] = value;
        array[i + 1] = buffer;
        if i == 0 {
            break;
        }
        i -= 1;
    }
}
