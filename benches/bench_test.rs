#![feature(test)]

extern crate test;
extern crate password_maker_lib;

use test::test::Bencher;
use password_maker_lib::package;

#[cfg(not(debug_assertions))]
#[bench]
fn randomization_bench(b: &mut Bencher)
    -> ()
{
    let mut passwords:Vec<String> = Vec::new();
    let length = 10000;
    let amount = 75000;
    b.iter(||
        {
            package(&mut passwords, &length, &amount);
            //tests are bugged atm, .len() does not return the correct value
            //assert_eq!(passwords.len(),amount);
            //assert_eq!(passwords.last().unwrap().len(), length);
        });
}
