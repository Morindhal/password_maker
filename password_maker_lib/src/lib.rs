extern crate rand;

use rand::{thread_rng,Rng};

pub fn package(passwords: &mut Vec<String>, length: &usize, amount: &usize)
{
    let mut rng = thread_rng();
    let mut randomized = String::new();
    
    let allowed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890+-!?=.,;:'*^";
    'ultra: loop
    {
        'outer: loop
        {
            randomized.push(allowed.chars().nth(rng.gen_range(0usize, allowed.len())).unwrap() );
            if randomized.len() >= *length
            {break 'outer;}
        }
        passwords.push(randomized.clone());
        randomized.clear();
        if passwords.len() >= *amount
        {break 'ultra;}
    }
}
