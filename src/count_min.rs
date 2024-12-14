use xxhash_rust::xxh3::xxh3_64;
use cityhash::city_hash_64;
use murmur3::murmur3_128_x64;

pub struct CountMinSketch {
    width: usize,
    depth: usize,
    table: Vec<Vec<u32>>,
    hash_functions: Vec<Box<dyn Fn(&[u8]) -> u64>>,
}

impl CountMinSketch {
    pub fn new(width: usize, depth: usize) -> Self {
        let table = vec![vec![0; width]; depth];
        let hash_functions: Vec<Box<dyn Fn(&[u8]) -> u64>> = vec![
            Box::new(|data| xxh3_64(data)),
            Box::new(|data| city_hash_64(data)),
            Box::new(|data| {
                let mut hash = [0u8; 16];
                murmur3_128_x64(data, 0, &mut hash).unwrap();
                u64::from_le_bytes([hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7]])
            }),
        ];
        CountMinSketch { width, depth, table, hash_functions }
    }

    pub fn add(&mut self, item: &[u8]) {
        for (i, hash_function) in self.hash_functions.iter().enumerate() {
            let hash = hash_function(item) as usize % self.width;
            self.table[i][hash] += 1;
        }
    }

    pub fn query(&self, item: &[u8]) -> u32 {
        let mut min_count = u32::MAX;
        for (i, hash_function) in self.hash_functions.iter().enumerate() {
            let hash = hash_function(item) as usize % self.width;
            min_count = min_count.min(self.table[i][hash]);
        }
        min_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_min_sketch() {
        let mut cms = CountMinSketch::new(100, 3);
        cms.add(b"hello");
        cms.add(b"hello");
        cms.add(b"world");

        assert_eq!(cms.query(b"hello"), 2);
        assert_eq!(cms.query(b"world"), 1);
        assert_eq!(cms.query(b"rust"), 0);
    }
}