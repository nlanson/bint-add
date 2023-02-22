use std::convert::TryInto;


struct FullAdder;

impl FullAdder {
    
    /// Full adder circuit addition implementation.
    /// 
    /// Takes in bit a, bit b and carry-in bit.
    /// Returns output bit and carry-out bit.
    fn add(a: bool, b: bool, cin: bool) -> (bool, bool) {
        (
            // DNF form circuit
            (a&b&cin) || (a&!b&!cin) || (!a&b&!cin) || (!a&!b&cin),
            (a&b&cin) || (a&b&!cin) || (a&!b&cin) || (!a&b&cin)
        )
    }

}

/// Binary integer implementation where N is the number of underlying bits.
#[derive(Debug, Eq, PartialEq)]
pub struct BinInteger<const N: usize>([bool; N]);

impl<const N: usize> BinInteger<N> {
    pub fn new(int: [bool; N]) -> BinInteger<N> {
        Self(int)
    }
}

impl<const N: usize> From<[u8; N]> for BinInteger<N> {
    fn from(int: [u8; N]) -> BinInteger<N> {
        let mut repr: [bool; N] = [false; N];
        for i in 0..N {
            repr[i] = int[i] != 0;
        }
        
        Self(repr)
    }
}

impl<const N: usize> From<Vec<bool>> for BinInteger<N> {
    fn from(v: Vec<bool>) -> BinInteger<N> {
        if v.len() != N {
            panic!("Length does not match")
        }
        
        Self(v.try_into().expect("Failed to convert"))
    }
}

impl<const N: usize> std::ops::Add for BinInteger<N> {
    type Output = Vec<bool>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out: Vec<bool> = vec![false; N+1];
        let mut carry: bool = false;
        for i in (0..N).rev() {
            (out[i+1], carry) = FullAdder::add(self.0[i], rhs.0[i], carry);
        }
        out[0] = carry;
        out
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn addition() {
        let a: BinInteger<8> = BinInteger::from([0,0,0,0,1,1,1,1]);
        let b: BinInteger<8> = BinInteger::from([0,0,0,0,0,0,0,1]);
        let result: BinInteger<9> = BinInteger::from(a + b);
        let expected: BinInteger<9> = BinInteger::from([0,0,0,0,1,0,0,0,0]);

        assert_eq!(result, expected);
    }
}
