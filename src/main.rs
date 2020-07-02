#[macro_use]
extern crate timeit;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, LinkedList};
#[macro_use]
extern crate ndarray;
use ndarray::prelude::*;


fn n_fibonacci(n: usize) -> Vec<u128> {
    let mut out: Vec<u128> = Vec::with_capacity(n);
    out.push(0);
    out.push(1);
    for i in 2..(n-2) {
        out.push(out[i-1] + out[i-2]);
    }
    out
}

fn cstep(n: u128) -> u128 {
    if n % 2 == 0 {
        n/2
    } else {
        3*n + 1
    }
}

fn collatz(n: u128) -> Vec<u128> {
    let mut out: Vec<u128> = Vec::with_capacity(3);
    out.push(n);
    out.push(cstep(out[0]));
    out.push(cstep(out[1]));
    for i in 3.. {
        if (out[i-3] == 4) && (out[i-2] == 2) && (out[i-1] == 1) {
            break;
        }
        out.push(cstep(out[i-1]));
    }
    out
}

fn collatz_map(n: u128, map: &mut HashMap<u128, Vec<u128>>) -> Vec<u128> {
    let mut out: Vec<u128> = Vec::with_capacity(3);
    out.push(n);
    out.push(cstep(out[0]));
    out.push(cstep(out[1]));
    for i in 3.. {
        if ((out[i-3] == 4) && (out[i-2] == 2) && (out[i-1] == 1)) {
            break;
        } else if map.contains_key(&out[i-1]) {
            out.append(&mut map[&out[i-1]].clone());
            break;
        } else {
            out.push(cstep(out[i-1]));
        }
    }
    map.insert(out[0], out.clone());
    out
}

fn collatz_test(n: usize) -> Vec<Vec<u128>> {
    (1..n).map(|x| x as u128).collect::<Vec<u128>>().par_iter().map(|x| collatz(*x)).collect::<Vec<Vec<u128>>>()
}

fn faster_collatz_test(n: u128) {
    timeit!({});
}




fn main() {
    timeit!({
        n_fibonacci(125);
    });

    /*
    let n: usize = 100000000;
    timeit!({
        collatz_test(n);
    });
    let seconds = timeit_loops!(1, {
        collatz_test(n);
    });
    println!("{} ms on average", seconds * 1000.0);
    for ray in collatz_test(n) {
        //println!("{:?}", ray);
    }*/

    
    let n = std::u16::MAX;
    let f = n as f64;
    let ms = timeit_loops!(30, {
        let mut collatz_set: HashMap<u128, Vec<u128>> = HashMap::new();
        for i in 1..n {
            // println!("{}%", 100.0 * i as f64 / f);
            collatz_map(i as u128, &mut collatz_set);
        }
    });
    println!("{} milliseconds on average.", ms * 1000.0);
}
