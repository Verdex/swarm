
use std::cmp;

pub fn lev(a : &[u8], b : &[u8]) -> usize {
    match (a, b) {
        ([], b) => b.len(),
        (a, []) => a.len(),
        ([a, a_rest @ .. ], [b, b_rest @ .. ]) if a == b => lev(a_rest, b_rest),
        ([_, a_rest @ .. ], [_, b_rest @ .. ]) => 1 + cmp::min( cmp::min( lev(a_rest, b), lev( a, b_rest ) ), lev(a_rest, b_rest ) ),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_handle_distance_zero() {
        let a : [u8; 3]= [1, 2, 3];
        let b : [u8; 3]= [1, 2, 3];

        assert_eq!( lev(&a, &b), 0 );
    }

    #[test]
    fn should_handle_distance_one() {
        let a : [u8; 3]= [1, 2, 3];
        let b : [u8; 3]= [1, 2, 1];

        assert_eq!( lev(&a, &b), 1 );
    }
}