
pub fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
    let mut i = 0;
    let mut j = 1;
    let mut max = 0;
    let mut zero_num = 0;
    if a[0] == 0 {zero_num += 1;} else {max += 1;}
    if a[1] == 0 {zero_num += 1;} else {max += 1;}

    0
}

fn main() {
    let a = vec![1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1];
    let k = 3;
    let res = longest_ones(a, k);
    println!("longest ones: {}", res);
}
