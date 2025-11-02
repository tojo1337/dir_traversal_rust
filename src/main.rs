use std::{env::current_dir, thread::available_parallelism};

use demo::{greet, traverse, traverse_mult};

fn main() {
    greet("Tojo".to_string());

    let mut len1 = 0;
    let mut len2 = 0;
    
    // Basic dir traversal
    if let Ok(entry_dir) = current_dir(){
        if let Some(entry_str) = entry_dir.to_str(){
            let dir_list = traverse(String::from(entry_str));
            // dir_list.iter().for_each(|value|{println!("{}", value)});
            len1 = dir_list.len();
        }
    }

    // Check the available parallelism
    let max_parallelism = available_parallelism();
    if let Ok(max_thread_non_zero) = max_parallelism {
        let max_thread_count = max_thread_non_zero.get();
        if let Ok(entry_dir) = current_dir(){
            if let Some(entry_str) = entry_dir.to_str(){
                let dir_list = traverse_mult(String::from(entry_str), max_thread_count);
                dir_list.iter().for_each(|value|{println!("{}", value)});
                len2 = dir_list.len();
            }
        }
    }

    println!("[*]Length of array when simple traversal is done : {}", len1);
    println!("[*]Length of array when multiple threads are added in the traversal : {}", len2);
}
