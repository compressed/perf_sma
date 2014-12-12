extern crate test;

use std::rand;
use std::rand::distributions::{IndependentSample, Range};
use test::Bencher;

const N: uint = 700_000u;

fn sma(avect: &Vec<f32>, num_per: uint) -> Vec<f32> {
    let num_ele = avect.len();
    let mut tout: Vec<f32> = Vec::with_capacity(num_ele);
    for ndx in range(0u, num_ele-1u) {
        let mut tsum = 0f32;
        let beg_range = [1u, ndx - num_per];
        let begndx = beg_range.iter().max().unwrap();
        for slicendx in range(*begndx, ndx) {
            tsum += avect[slicendx];
        }
        tout.push(tsum / num_per.to_f32().unwrap());
    }
    tout
}

#[bench]
fn bench_sma_100(b: &mut Bencher) {
    let mut rng = rand::task_rng();
    let between = Range::new(0f32, 1.);
    let f32_data: Vec<f32> = Vec::from_fn(N,
        |_| between.ind_sample(&mut rng));

    b.iter(|| {
        sma(&f32_data, 100);
    })
}
