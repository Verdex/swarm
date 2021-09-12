
pub fn noise_1d( mut input : u64 ) -> u64 {
    input = input.wrapping_add(17);
    input = input ^ input.wrapping_shl(13);
    input = input ^ input.wrapping_shr(7);
    input = input ^ input.wrapping_shl(17);
    input
}

pub fn noise_2d( x : u64, y : u64 ) -> u64 {
    noise_1d(x) ^ noise_1d( y.wrapping_mul( 3 ) )
}

pub fn noise_3d( x : u64, y : u64, z : u64 ) -> u64 {
    noise_1d(x) ^ noise_1d( y.wrapping_mul( 3 ) ) ^ noise_1d( z.wrapping_mul( 7 ) )
}

pub fn noise_4d( x : u64, y : u64, z : u64, w : u64 ) -> u64 {
    noise_1d(x) ^ noise_1d( y.wrapping_mul( 3 ) ) ^ noise_1d( z.wrapping_mul( 7 ) ) ^ noise_1d( w.wrapping_mul( 11 ) )
}

pub fn to_index(input : u64, lub : usize) -> usize {
    input.wrapping_rem( lub as u64 ) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_properly_to_range() {
        assert_eq!( to_index(0, 5), 0 );
        assert_eq!( to_index(500, 5), 0 );
        assert_eq!( to_index(4, 5), 4 );
        assert_eq!( to_index(6, 5), 1 );
    }

}