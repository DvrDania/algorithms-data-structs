#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<i32>,
    right: Option<i32>,
}

pub fn sort(arr: &mut Vec<i32>) {
    let i = ((arr.len() as f32 / 2 as f32).floor()) as usize;
    let node_for_i = Node {
        value: arr[i],
        left: None,
        right: None,
    };
    println!("{:#?}", node_for_i);
}
