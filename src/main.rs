use std::io::{self, Write};
use rand;

fn check_win(arr: &[i32]) -> bool {
    let first_value = arr[0];
    for value in arr.iter() {
        if *value != first_value {
            return false;
        }
    }
    return true;
}

fn main() {
    print!("Enter the value of n : ");
    io::stdout().flush().unwrap();
    
    let mut str_n = String::new();
    io::stdin().read_line(&mut str_n).unwrap();
    let n: usize = str_n.trim().parse().unwrap();
    
    let mut arr = vec![1i32; n];
    for i in 0usize..n {
        if rand::random() {
            arr[i] = 0;
        }
    }
    
    println!("{:?}", arr);
    println!("{}", check_win(&arr));
}
