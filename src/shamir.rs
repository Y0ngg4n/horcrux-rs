//Heavy lifting credit goes to:
//https://github.com/Nebulosus/shamir/blob/master/src/lib.rs
//https://github.com/jesseduffield/horcrux/blob/master/pkg/shamir/shamir.go
//https://github.com/hashicorp/vault/blob/master/shamir/shamir.go
//https://github.com/dsprenkels/sss-rs/blob/master/src/lib.rs

use crate::tables::{EXP_TABLE, LOG_TABLE};
use rand::{
    rngs::{OsRng, StdRng, ThreadRng},
    seq::SliceRandom,
    thread_rng, Rng, SeedableRng,
};

const SHARE_OVERHEAD: usize = 1;


struct Polynomial<'a> {
    coefficients: &'a [u8],
}

impl<'a> Polynomial<'a> {
    pub fn new(&self, intercept: u8, degree: u8) -> Self {
        Self {
            coefficients: &mut [degree + 1],
        };
    
        // Ensure the intercept is set
        self.coefficients[0] = intercept;
    
        // Assign random coefficients to the polynomial
        let mut rng = OsRng;
        rng.try_fill(&mut polynomial.coefficients[1..])?;
    
        Self
    }

    fn evaluate(&self, x: u8) -> u8 {
        // Special case the origin
        if x == 0 {
            return self.coefficients[0];
        }

        // Compute the polynomial value using Horner's method.
        let degree = self.coefficients.len() - 1;
        let mut out = self.coefficients[degree];
        for i in (0..degree).rev() {
            let coeff = self.coefficients[i];
            out = add(multiply(out, x), coeff);
        }
        out
    }
}

fn divide(a: u8, b: u8) -> u8 {
    if b == 0 {
        panic!("divide by zero");
    }

    let good_value: u8;
    let zero: u8 = 0;
    let log_a = LOG_TABLE[a as usize];
    let log_b = LOG_TABLE[b as usize];
    let difference = (log_a as i32 - log_b as i32) % 255;
    let mut ret = EXP_TABLE[difference as usize];

    // Ensure we return zero if a is zero but aren't subject to timing attacks
    good_value = ret;

    if a == 0 {
        ret = zero;
    } else {
        ret = good_value;
    }
    ret
}



// add combines two numbers in GF(2^8)
// This can also be used for subtraction since it is symmetric.
fn add(a: u8, b: u8) -> u8 {
    a ^ b
}

fn multiply(a: u8, b: u8) -> u8 {
    let log_a = LOG_TABLE[a as usize];
    let log_b = LOG_TABLE[b as usize];
    let sum = (log_a as u16 + log_b as u16) % 255;

    let mut good_value: u8 = 0;
    let zero: u8 = 0;

    let mut ret: u8 = EXP_TABLE[sum as usize];

    good_value = ret;

    //To avoid timing attacks, we must return zero if either a or b are zero.
    let a_is_zero: bool = subtle::ConstantTimeEq::ct_eq(&a, &0).into();
    let b_is_zero: bool = subtle::ConstantTimeEq::ct_eq(&b, &0).into();

    if a_is_zero {
        ret = zero;
    } else {
        ret = good_value;
    }

    if b_is_zero {
        ret = zero;
    } else {
        good_value = zero;
    }
    ret
}

fn split<'a>(secret: &[u8], parts: &'a [u8], threshold: u8) -> Result<Vec<Vec<u8>>, rand::Error> {
    //TODO Sanity checks

    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    // // Create a new random number generator with the generated seed
    let mut rng = StdRng::seed_from_u64(seed);
    let mut x_coordinates: Vec<u8> = (1..=255).collect();
    x_coordinates.shuffle(&mut rng);

    // Allocate the output array, initialize the final byte
    // of the output with the offset. The representation of each
    // output is {y1, y2, .., yN, x}.
    let output: Vec<Vec<u8>> = (0..parts.len())
        .map(|index: usize| {
            let mut slice: Vec<u8> = vec![0u8; secret.len() + 1];
            slice[index] = x_coordinates[index] + 1;
            slice
        })
        .collect();


    for (idx, &val) in secret.iter() {
        let p = Polynomial::new(val, (threshold - 1));

        // Generate a `parts` number of (x,y) pairs
        // We cheat by encoding the x value once as the final index,
        // so that it only needs to be stored once.
        for i in 0..parts {
            let x = (x_coordinates[i] as u8) + 1;
            let y = p.evaluate(x);
            out[i][idx] = y;
        }
    }

    // secret
    //     .iter()
    //     .enumerate()
    //     .map(|(index, &val)| {
    //         (0..parts)
    //             .map(|i| {
    //                 let x = x_coordinates[i] + 1;
    //                 let polynomial: Polynomial::new(val, threshold - 1);
    //                     .and_then(|p| Ok(p.evaluate(x)))
    //                     .unwrap_or_default()
    //             })
    //             .collect()
    //     })
    //     .fold(out, |mut acc, ys| {
    //         ys.into_iter().enumerate().for_each(|(i, y)| acc[i].push(y));
    //         acc
    //     });
}
