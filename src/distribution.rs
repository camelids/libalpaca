//! Provides functions to sample objects' count and size from a
//! probability distribution.
use rand::Rng;
use rand::distributions::{LogNormal, Sample};

/// Parameters for a log-normal distribution.
struct DistParams {
    mean: f64,
    std_dev: f64,
}

// Hardcoded parameters for log-normal distributions.
const OBJECT_COUNT_DIST: DistParams = DistParams { mean: 0.4,
                                                   std_dev: 2.0,
                                                 };
const OBJECT_SIZE_DIST: DistParams = DistParams { mean: 8.8,
                                                  std_dev: 1.0,
                                                };
const HTML_SIZE_DIST: DistParams = DistParams { mean: 8.8,
                                                std_dev: 1.0,
                                              };

// Number of tries per sample. If no sampled number satisfies a specified
// threshold after `SAMPLE_LIMIT` tries the sampling function returns Err.
const SAMPLE_LIMIT: u8 = 30;

/// Samples an `usize` number according to the log-normal distribution
/// with the specified parameters.
///
/// # Arguments
///
/// `rng` - Random number generator.
/// `params` - Parameters (mean and standard deviation) for the log-normal
///            distribution.
/// `ge` - Threshold: the sampled number should be greater than or equal to
///        `ge`.
///
/// # Returns
/// The sampled number. The function tries `SAMPLE_LIMIT` times to sample
/// a number satisfying the threshold. If no number satisfies it, it returns
/// Err(()).
fn sample<R: Rng>(rng: &mut R, params: DistParams, ge: usize)
        -> Result<usize, ()> {

    let mut dist = LogNormal::new(params.mean, params.std_dev);

    for _ in 0..SAMPLE_LIMIT {
        let x = dist.sample(rng) as usize;
        if x >= ge {
            return Ok(x);
        }
    }

    Err(())
}

/// Samples a new object count.
///
/// Samples a new object count from a log-normal distribution
/// specified by the distribution parameters OBJECT_COUNT_DIST.
pub fn sample_object_count<R: Rng>(rng: &mut R, ge: usize) -> Result<usize, ()> {

    sample(rng, OBJECT_COUNT_DIST, ge)
}

/// Samples the size of an HTML page.
///
/// Samples the size of an HTML page from a log-normal distribution
/// specified by the distribution parameters HTML_SIZE_DIST.
pub fn sample_html_size<R: Rng>(rng: &mut R, ge: usize) -> Result<usize, ()> {

    sample(rng, HTML_SIZE_DIST, ge)
}

/// Samples a new object count.
///
/// Samples the sizes of n objects from a log-normal distribution
/// specified by the distribution parameters OBJECT_SIZE_DIST.
pub fn sample_object_sizes<R: Rng>(rng: &mut R, n: usize, ge: usize)
        -> Result<Vec<usize>, ()> {

    (0..n).into_iter()
          .map(|_| sample(rng, OBJECT_SIZE_DIST, ge))
          .collect()
}
