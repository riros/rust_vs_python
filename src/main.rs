#![allow(unused)]

use rayon::prelude::*;

#[macro_use]
extern crate itertools;
extern crate num;
extern crate rand;

use itertools::{Itertools, MultiProduct};
use rand::Rng;

fn main() {
    const K: u32 = 3;
    const M: u128 = 1000;
    let mut ni: Vec<Vec<u32>> = Vec::new();
    let mut rng = rand::thread_rng();
    let imax: u64 = num::pow(10, K as usize);

    println!("load");
    for _ in (0u32..K) {
        let mut arr: Vec<u32> = Vec::new();

        for _ in (0..imax) {
            arr.push(rng.gen_range(1, num::pow(10, 9)));
        }

        ni.push(arr);
    }

    println!("calc");
    println!(
        "total {}",
        ni.into_iter()
            .multi_cartesian_product()
            .map(|x: Vec<u32>| -> u16 {
                (x.into_iter()
                    .map(|y: u32| num::pow(y as u128, 2))
                    .sum::<u128>()
                    % M) as u16
            })
            .collect::<Vec<u16>>()
            .par_iter()
            .reduce_with(|a, b| (if a >= b { a } else { b }))
            .unwrap()
    );

    //    print(reduce((lambda x, y: x if x >= y else y), map(lambda x: sum(map(mul, x, x)) % M, product(*Ni))))
}
