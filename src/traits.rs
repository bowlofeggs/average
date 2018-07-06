/// Estimate a statistic of a sequence of numbers ("population").
pub trait Estimate {
    /// Add an observation sampled from the population.
    fn add(&mut self, x: f64);

    /// Estimate the statistic of the population.
    fn estimate(&self) -> f64;
}

/// Merge another sample into this one.
pub trait Merge {
    fn merge(&mut self, other: &Self);
}

/// Calculate the multinomial variance. Relevant for histograms.
#[inline(always)]
fn multinomal_variance(n: f64, n_tot_inv: f64) -> f64 {
    n * (1. - n * n_tot_inv)
}

/// Get the bins and ranges from a histogram.
pub trait Histogram:
    where for<'a> &'a Self: IntoIterator<Item = ((f64, f64), u64)>
{
    /// Return the bins of the histogram.
    fn bins(&self) -> &[u64];

    /// Estimate the variance for the given bin.
    ///
    /// The square root of this estimates the error of the bin count.
    #[inline]
    fn variance(&self, bin: usize) -> f64 {
        let count = self.bins()[bin];
        let sum: u64 = self.bins().iter().sum();
        multinomal_variance(count as f64, 1./(sum as f64))
    }

    /// Return an iterator over the bins normalized by the bin widths.
    #[inline]
    fn normalized_bins(&self) -> IterNormalized<<&Self as IntoIterator>::IntoIter> {
        IterNormalized { histogram_iter: self.into_iter() }
    }

    /// Return an iterator over the bin widths.
    #[inline]
    fn widths(&self) -> IterWidths<<&Self as IntoIterator>::IntoIter> {
        IterWidths { histogram_iter: self.into_iter() }
    }

    /// Return an iterator over the bin centers.
    #[inline]
    fn centers(&self) -> IterBinCenters<<&Self as IntoIterator>::IntoIter> {
        IterBinCenters { histogram_iter: self.into_iter() }
    }

    /// Return an iterator over the bin variances.
    ///
    /// This is more efficient than calling `variance()` for each bin.
    #[inline]
    fn variances(&self) -> IterVariances<<&Self as IntoIterator>::IntoIter> {
        let sum: u64 = self.bins().iter().sum();
        IterVariances {
            histogram_iter: self.into_iter(),
            sum_inv: 1./(sum as f64)
        }
    }
}

/// Iterate over the bins normalized by bin width.
pub struct IterNormalized<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    histogram_iter: T,
}

impl<T> Iterator for IterNormalized<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        self.histogram_iter.next().map(|((a, b), count)| (count as f64) / (b - a))
    }
}

/// Iterate over the widths of the bins.
pub struct IterWidths<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    histogram_iter: T,
}

impl<T> Iterator for IterWidths<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        self.histogram_iter.next().map(|((a, b), _)| b - a)
    }
}

/// Iterate over the bin centers.
pub struct IterBinCenters<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    histogram_iter: T,
}

impl<T> Iterator for IterBinCenters<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        self.histogram_iter.next().map(|((a, b), _)| 0.5 * (a + b))
    }
}

/// Iterate over the variances.
pub struct IterVariances<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    histogram_iter: T,
    sum_inv: f64,
}

impl<T> Iterator for IterVariances<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        self.histogram_iter.next()
            .map(|(_, n)| multinomal_variance(n as f64, self.sum_inv))
    }
}
