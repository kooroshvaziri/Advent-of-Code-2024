use libm::*;
use std::collections::HashMap;

fn avent_blink(list: &Vec<usize>, blinks: i32)->usize {
    let mut source: HashMap<usize, usize> = HashMap::new();
    
    for i in 0..list.len() {
        let num = list[i];
        source.insert(num, source.get(&num).unwrap_or(&0)+1);
    }

    for _ in 0..blinks {
        let mut result: HashMap<usize, usize> = HashMap::new();
        for (stone, count) in source {
            let len = log10(stone as f64).floor() as i32+1;
            if len % 2 == 0 {
                //even length
                let divider = 10_usize.pow((len/2) as u32);
                let first = stone / divider;
                result.insert(first, result.get(&first).unwrap_or(&0)+count);
    
                let second = stone % divider;
                result.insert(second, result.get(&second).unwrap_or(&0)+count);
            } else if stone == 0 {
                let new_val: usize = 1;
                result.insert(new_val, result.get(&new_val).unwrap_or(&0)+count);
            } else {
                let new_val: usize = stone * 2024;
                result.insert(new_val, result.get(&new_val).unwrap_or(&0)+count);
            }
        }
        source = result.clone();
    }
    return source.values().sum::<usize>();
}


fn main() {
    let list: Vec<usize> = vec![475449,2599064,213,0,2,65,5755,51149];
    
    let blinks: i32 = 25;
    let count = avent_blink(&list, blinks);
    println!("Number of stones after {:?} blinks is {:?}!", blinks, count);

    let blinks: i32 = 75;
    let count = avent_blink(&list, blinks);
    println!("Number of stones after {:?} blinks is {:?}!", blinks, count);
}
