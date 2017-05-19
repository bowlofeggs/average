use core;

use conv::ApproxFrom;

/// Represent the arithmetic mean and the variance of a sequence of numbers.
///
/// Everything is calculated iteratively using constant memory, so the sequence
/// of numbers can be an iterator. The used algorithms try to avoid numerical
/// instabilities.
///
/// ```
/// use average::Average;
///
/// let a: Average = (1..6).map(Into::into).collect();
/// assert_eq!(a.mean(), 3.0);
/// assert_eq!(a.sample_variance(), 2.5);
/// ```
#[derive(Debug, Clone)]
pub struct Average {
    /// Average value.
    avg: f64,
    /// Number of samples.
    n: u64,
    /// Intermediate sum of squares for calculating the variance.
    v: f64,
}

impl Average {
    /// Create a new average.
    pub fn new() -> Average {
        Average { avg: 0., n: 0, v: 0. }
    }

    /// Add a sample to the sequence of which the average is calculated.
    pub fn add(&mut self, sample: f64) {
        // This algorithm introduced by Welford in 1962 trades numerical
        // stability for a division inside the loop.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        self.n += 1;
        let delta = sample - self.avg;
        self.avg += delta / f64::approx_from(self.n).unwrap();
        self.v += delta * (sample - self.avg);
    }

    /// Determine whether the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Estimate the mean of the sequence.
    pub fn mean(&self) -> f64 {
        self.avg
    }

    /// Return the number of elements in the sequence.
    pub fn len(&self) -> u64 {
        self.n
    }

    /// Calculate the unbiased sample variance of the sequence.
    ///
    /// This assumes that the sequence consists of samples of a larger population.
    pub fn sample_variance(&self) -> f64 {
        if self.n < 2 {
            return 0.;
        }
        self.v / f64::approx_from(self.n - 1).unwrap()
    }

    /// Calculate the population variance of the sequence.
    ///
    /// This assumes that the sequence consists of the entire population.
    pub fn population_variance(&self) -> f64 {
        if self.n < 2 {
            return 0.;
        }
        self.v / f64::approx_from(self.n).unwrap()
    }

    /// Estimate the standard error of the mean of the sequence.
    pub fn error(&self) -> f64 {
        if self.n == 0 {
            return 0.;
        }
        (self.sample_variance() / f64::approx_from(self.n).unwrap()).sqrt()
    }

    /// Merge the average of another sequence into this one.
    ///
    /// ```
    /// use average::Average;
    ///
    /// let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    /// let (left, right) = sequence.split_at(3);
    /// let avg_total: Average = sequence.iter().map(|x| *x).collect();
    /// let mut avg_left: Average = left.iter().map(|x| *x).collect();
    /// let avg_right: Average = right.iter().map(|x| *x).collect();
    /// avg_left.merge(&avg_right);
    /// assert_eq!(avg_total.mean(), avg_left.mean());
    /// assert_eq!(avg_total.sample_variance(), avg_left.sample_variance());
    /// ```
    pub fn merge(&mut self, other: &Average) {
        // This algorithm was proposed by Chan et al. in 1979.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        let delta = other.avg - self.avg;
        let len_self = f64::approx_from(self.n).unwrap();
        let len_other = f64::approx_from(other.n).unwrap();
        let len_total = len_self + len_other;
        self.n += other.n;
        self.avg = (len_self * self.avg + len_other * other.avg) / len_total;
        // Chan et al. use
        //
        //     self.avg += delta * len_other / len_total;
        //
        // instead but this results in cancelation if the number of samples are similar.
        self.v += other.v + delta*delta * len_self * len_other / len_total;
    }
}

impl core::default::Default for Average {
    fn default() -> Average {
        Average::new()
    }
}

impl core::iter::FromIterator<f64> for Average {
    fn from_iter<T>(iter: T) -> Average
        where T: IntoIterator<Item=f64>
    {
        let mut a = Average::new();
        for i in iter {
            a.add(i);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge() {
        let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
        for mid in 0..sequence.len() {
            let (left, right) = sequence.split_at(mid);
            let avg_total: Average = sequence.iter().map(|x| *x).collect();
            let mut avg_left: Average = left.iter().map(|x| *x).collect();
            let avg_right: Average = right.iter().map(|x| *x).collect();
            avg_left.merge(&avg_right);
            assert_eq!(avg_total.n, avg_left.n);
            assert_eq!(avg_total.avg, avg_left.avg);
            assert_eq!(avg_total.v, avg_left.v);
        }
    }
}