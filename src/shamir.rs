use rand::{Rng, rngs::OsRng};
use crate::tables::{EXP_TABLE, LOG_TABLE};
const SHARE_OVERHEAD: usize = 1;


struct Polynomial<'a> {
    coefficients: &'a mut [u8],
}

impl<'a> Polynomial<'a> {
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
            out = add(mult(out, x), coeff);
        }
        out
    }
}



fn make_polynomial<'a>(intercept: u8, degree: u8) -> Result<Polynomial<'a>, rand::Error> {
    // Create a wrapper
    
    let mut polynomial = Polynomial {
        coefficients: &mut [degree + 1],
    };

    // Ensure the intercept is set
    polynomial.coefficients[0] = intercept;

    // Assign random coefficients to the polynomial
    let mut rng = OsRng;
    rng.try_fill(&mut polynomial.coefficients[1..])?;

    Ok(polynomial)
}




// add combines two numbers in GF(2^8)
// This can also be used for subtraction since it is symmetric.
fn add(a:u8, b:u8) -> u8 {
	a ^ b
}


fn mult(a: u8, b: u8) -> u8 {
    let log_a = LOG_TABLE[a as usize];
    let log_b = LOG_TABLE[b as usize];
    let sum = (log_a as u16 + log_b as u16) % 255;

    let ret = EXP_TABLE[sum as usize];

    let good_val: u8 = ret;
    let zero: u8 = 0;

    let a_eq_zero = subtle::ConstantTimeByteEq(a, 0);
    let b_eq_zero = subtle::ConstantTimeByteEq(b, 0);

    let ret = (a_eq_zero & zero) | ((!a_eq_zero) & good_val);
    let ret = (b_eq_zero & zero) | ((!b_eq_zero) & {
        let good_val = zero;
        good_val
    });

    ret
}