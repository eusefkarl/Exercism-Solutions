use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    position: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a> 
        where 
        Key: AsRef<[u8]> + ?Sized,
        {
        let cast: &[u8] = key.as_ref();
        Xorcism{key: cast, position: 0}
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut(){
            *byte ^= self.key[self.position % self.key.len()];
            self.position+=1;
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
        where 
        Data: IntoIterator,
        Data::Item: Borrow<u8>, {
        let input = data.into_iter();
        input.map(|byte|{
            let b = *byte.borrow();
            let k = self.key[self.position % self.key.len()];
            let res = b^k;
            self.position += 1;
            res
        })
    }
}
