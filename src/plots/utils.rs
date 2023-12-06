use std::cmp;
use std::collections::btree_map::Range;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Histogram {
    num_buckets: u64,
    samples: BTreeMap<u64, u64>,
    min: u64,
    max: u64,
}

impl Histogram {
    pub fn new(num_buckets: u64) -> Self {
        assert!(num_buckets > 0);
        Self {
            num_buckets,
            samples: Default::default(),
            min: Default::default(),
            max: Default::default(),
        }
    }

    pub fn add(&mut self, sample: u64) {
        if sample < self.min {
            self.min = sample
        }
        if sample > self.max {
            self.max = sample
        }

        *self.samples.entry(sample).or_insert(0) += 1;
    }

    /// Get an iterator over this histogram's buckets.
    pub fn buckets(&self) -> Buckets {
        Buckets {
            histogram: self,
            index: 0,
        }
    }
}

impl fmt::Display for Histogram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;

        let num_samples: u64 = self.samples.values().sum();
        writeln!(f, "# Number of samples = {}", num_samples)?;
        if num_samples == 0 {
            return Ok(());
        }

        writeln!(f, "# Min = {}", self.min)?;
        writeln!(f, "# Max = {}", self.max)?;
        writeln!(f, "#")?;

        let max_bucket_count = self.buckets().map(|b| b.count()).fold(0, cmp::max);

        const WIDTH: u64 = 50;
        let count_per_char = cmp::max(max_bucket_count / WIDTH, 1);

        writeln!(f, "# Each ∎ is a count of {}", count_per_char)?;
        writeln!(f, "#")?;

        let mut count_str = String::new();

        let widest_count = self.buckets().fold(0, |n, b| {
            count_str.clear();
            write!(&mut count_str, "{}", b.count()).unwrap();
            cmp::max(n, count_str.len())
        });

        let mut end_str = String::new();
        let widest_range = self.buckets().fold(0, |n, b| {
            end_str.clear();
            write!(&mut end_str, "{}", b.end()).unwrap();
            cmp::max(n, end_str.len())
        });

        let mut start_str = String::with_capacity(widest_range);

        for bucket in self.buckets() {
            start_str.clear();
            write!(&mut start_str, "{}", bucket.start()).unwrap();
            for _ in 0..widest_range - start_str.len() {
                start_str.insert(0, ' ');
            }

            end_str.clear();
            write!(&mut end_str, "{}", bucket.end()).unwrap();
            for _ in 0..widest_range - end_str.len() {
                end_str.insert(0, ' ');
            }

            count_str.clear();
            write!(&mut count_str, "{}", bucket.count()).unwrap();
            for _ in 0..widest_count - count_str.len() {
                count_str.insert(0, ' ');
            }

            write!(f, "{} .. {} [ {} ]: ", start_str, end_str, count_str)?;
            for _ in 0..bucket.count() / count_per_char {
                write!(f, "∎")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Buckets<'a> {
    histogram: &'a Histogram,
    index: u64,
}

impl<'a> Iterator for Buckets<'a> {
    type Item = Bucket<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.histogram.num_buckets {
            return None;
        }

        let range = self.histogram.max - self.histogram.min;
        let range = range + (range % self.histogram.num_buckets);

        let bucket_size = range / self.histogram.num_buckets;
        let bucket_size = cmp::max(1, bucket_size);

        let start = self.histogram.min + self.index * bucket_size;
        let end = self.histogram.min + (self.index + 1) * bucket_size;

        self.index += 1;

        Some(Bucket {
            start,
            end,
            range: if self.index == self.histogram.num_buckets {
                self.histogram.samples.range(start..)
            } else {
                self.histogram.samples.range(start..end)
            },
        })
    }
}

#[derive(Clone)]
pub struct Bucket<'a> {
    start: u64,
    end: u64,
    range: Range<'a, u64, u64>,
}

impl<'a> Bucket<'a> {
    pub fn count(&self) -> u64 {
        self.range.clone().map(|(_, count)| count).sum()
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;

    #[test]
    fn num_buckets() {
        let mut histo = Histogram::new(10);
        for i in 0..100 {
            histo.add(i);
        }
        assert_eq!(histo.buckets().count(), 10);
    }

    #[test]
    fn bucket_count() {
        let mut histo = Histogram::new(1);
        for i in 0..10 {
            histo.add(i);
        }
        assert_eq!(histo.buckets().count(), 1);
        assert_eq!(histo.buckets().next().unwrap().count(), 10);
    }

    #[test]
    fn bucket_count_one() {
        let mut histo = Histogram::new(1);
        histo.add(1);
        assert_eq!(histo.buckets().count(), 1);
        assert_eq!(histo.buckets().next().unwrap().count(), 1);
    }

    #[test]
    fn overflow_panic() {
        use std::string::ToString;

        let mut histo = Histogram::new(1);
        histo.add(99);
        histo.to_string();
    }

    #[test]
    fn sum_of_bucket_counts_is_total_count() -> () {
        let buckets = 10;
        let samples: Vec<u64> = (0..100).collect();
        if buckets == 0 {
            return;
        }

        let len = samples.len();
        let mut histo = Histogram::new(buckets);
        for s in samples {
            histo.add(s);
        }

        assert_eq!(
            len as u64,
            histo.samples.values().cloned().sum::<u64>(),
            "samples.values() count should be correct"
        );
        assert_eq!(
            len as u64,
            histo.buckets().map(|b| b.count()).sum::<u64>(),
            "sum of buckets counts should be correct"
        );
    }

    #[test]
    fn actual_buckets_should_be_less_than_or_equal_num_buckets() {
        let buckets = 10;
        let samples: Vec<u64> = (0..100).collect();
        if buckets == 0 {
            return;
        }

        let mut histo = Histogram::new(buckets);
        for s in samples {
            histo.add(s);
        }

        assert!(
            histo.buckets().count() as u64 <= buckets,
            "should never have more than expected number of buckets"
        );
    }

    #[test]
    fn bucket_ranges_should_be_correct() -> () {
        let buckets = 10;
        let samples: Vec<u64> = (0..100).collect();
        if buckets == 0 {
            return;
        }

        let mut histo = Histogram::new(buckets);
        for s in samples {
            histo.add(s);
        }

        histo.buckets().fold(None, |min_max, bucket| {
            let bucket_range = bucket.end() - bucket.start();
            let (min, max) = min_max.unwrap_or((bucket_range, bucket_range));
            let min = cmp::min(min, bucket_range);
            let max = cmp::max(max, bucket_range);
            assert!(max - min <= 1);
            Some((min, max))
        });
    }

    #[test]
    fn formatting_should_never_panic() -> () {
        let buckets = 10;
        let samples: Vec<u64> = (0..100).collect();
        use std::string::ToString;

        if buckets == 0 {
            return;
        }

        let mut histo = Histogram::new(buckets);
        for s in samples {
            histo.add(s);
        }

        histo.to_string();
    }
}
