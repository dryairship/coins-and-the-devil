use std::io::{self, Write};
use std::collections::VecDeque;
use rand;

fn check_win(arr: &VecDeque<i32>) -> bool {
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
    
    let mut arr = VecDeque::<i32>::with_capacity(n);
    for _i in 0usize..n {
        if rand::random() {
            arr.push_back(0);
            // println!("Pushing 0");
        } else {
            arr.push_back(1);
            // println!("Pushing 1");
        }
    }
    
    println!("{:?}", arr);
    let mut new_first_value: i32;
    while !check_win(&arr) {
        let val = arr.pop_back();
        if val == Some(1) {
            new_first_value = 0;
            if arr[0]==0 {
                new_first_value = 1;
            }
            println!("Devil's turn. Devil chose : {}", new_first_value);
        }else{
            print!("Your turn. Enter your choice (0/1) : ");
            let mut ans_n = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans_n).unwrap();
            new_first_value = ans_n.trim().parse().unwrap();
        }
        arr.push_front(new_first_value);
        println!("{:?}", arr);
    }
    println!("Congratulations, you won!");
}
