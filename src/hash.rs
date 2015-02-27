use std::hash::{Hash,Hasher};
use std::iter::Iterator;

/// Returns an iterator of indexes for the given element with a maximum
/// size. This uses [double
/// hashing](http://www.eecs.harvard.edu/~kirsch/pubs/bbbf/esa06.pdf), allowing
/// for multiple indexes to be created from only two full runs through
/// SipHash2-4.
pub fn indexes<E: Hash>(e: &E, max: usize) -> Index {
    let mut h1 = FnvHasher(0xcbf29ce484222325);
    e.hash(&mut h1);
    let hash1 = h1.finish();

    let mut h2 = FnvaHasher(0xcbf29ce484222325);
    e.hash(&mut h2);
    let hash2 = h2.finish();

    Index {
        h1: hash1,
        h2: hash2,
        max: max as u64,
        i: 0,
    }
}

struct Index {
    h1: u64,
    h2: u64,
    max: u64,
    i: u64,
}

impl Iterator for Index {
    type Item = usize;

    #[inline(always)]
    fn next(&mut self) -> Option<usize> {
        self.i += 1;
        Some(((self.h1 + self.i * self.h2) % self.max) as usize)
    }
}

struct FnvHasher(u64);

impl Hasher for FnvHasher {
    fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;
        for byte in bytes {
            hash = hash * 0x100000001b3;
            hash = hash ^ (*byte as u64);
        }
        *self = FnvHasher(hash);
    }
    fn finish(&self) -> u64 { self.0 }
}

struct FnvaHasher(u64);

impl Hasher for FnvaHasher {
    fn write(&mut self, bytes: &[u8]) {
        let FnvaHasher(mut hash) = *self;
        for byte in bytes {
            hash = hash ^ (*byte as u64);
            hash = hash * 0x100000001b3;
        }
        *self = FnvaHasher(hash);
    }
    fn finish(&self) -> u64 { self.0 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn double_hashing() {
        let v: Vec<usize> = indexes(&"whee", 100).take(10).collect();

        assert_eq!(v, vec![2, 27, 52, 77, 86, 11, 36, 61, 86, 95]);
    }
}
