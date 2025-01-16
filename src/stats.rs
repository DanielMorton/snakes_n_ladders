use thiserror::Error;

/// Custom error types for the octopus simulation
#[derive(Error, Debug)]
pub enum SimulationError {
    #[error("Empty move array: no simulation data available")]
    EmptyMoveArray,

    #[error("Invalid target value: {0} must be positive")]
    InvalidTarget(i64),

    #[error("Invalid iteration count: {0} must be greater than zero")]
    InvalidIterationCount(u64),

    #[error("Statistical calculation error: {0}")]
    StatisticalError(String),
}

/// Statistical metrics for simulation results
#[derive(Debug)]
pub struct SimulationStats {
    min_moves: u64,
    max_moves: u64,
    min_moves_fraction: f64,
    mean: f64,
    variance: f64,
    std_deviation: f64,
    coeff_variation: f64,
    skewness: f64,
    kurtosis: f64,
    median: f64,
}

impl SimulationStats {
    /// Calculate statistical metrics from a slice of move counts
    pub(crate) fn from_values(values: &[u64]) -> Result<Self, SimulationError> {
        if values.is_empty() {
            return Err(SimulationError::EmptyMoveArray);
        }

        let min_moves = *values.iter().min().unwrap(); // Safe because we checked for empty
        let max_moves = *values.iter().max().unwrap();
        let min_moves_count = values.iter().filter(|&m| m == &min_moves).count();
        let count = values.len() as f64;

        let mean = calculate_mean(values)?;
        let variance = calculate_variance(values, mean)?;
        let std_deviation = variance.sqrt();

        if std_deviation == 0.0 {
            return Err(SimulationError::StatisticalError(
                "Standard deviation is zero".to_string(),
            ));
        }
        let median = calculate_median(values);

        Ok(SimulationStats {
            min_moves,
            max_moves,
            min_moves_fraction: (min_moves_count as f64) / count,
            mean,
            variance,
            std_deviation,
            coeff_variation: std_deviation / mean,
            skewness: calculate_skewness(values, mean, std_deviation)?,
            kurtosis: calculate_kurtosis(values, mean, std_deviation)?,
            median,
        })
    }

    /// Print all statistical metrics
    pub(crate) fn print(&self) {
        println!("Shortest Path Length: {}", self.min_moves);
        println!("Longest Observed Path Length: {}", self.max_moves);
        println!("Shortest Path Fraction: {}", self.min_moves_fraction);
        println!("Mean moves: {}", self.mean);
        println!("Variance: {}", self.variance);
        println!("Standard deviation: {}", self.std_deviation);
        println!("Coefficient of Variation: {}", self.coeff_variation);
        println!("Skew: {}", self.skewness);
        println!("Kurtosis: {}", self.kurtosis);
        println!("Median: {}", self.median)
    }

    pub(crate) fn header(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{}",
            stringify!(min_moves),
            stringify!(max_moves),
            stringify!(min_moves_fraction),
            stringify!(mean),
            stringify!(variance),
            stringify!(std_deviation),
            stringify!(coeff_variation),
            stringify!(skewness),
            stringify!(kurtosis),
            stringify!(median)
        )
    }

    pub(crate) fn write(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{}",
            self.min_moves,
            self.max_moves,
            self.min_moves_fraction,
            self.mean,
            self.variance,
            self.std_deviation,
            self.coeff_variation,
            self.skewness,
            self.kurtosis,
            self.median
        )
    }
}

// Helper functions for statistical calculations
pub fn calculate_mean(values: &[u64]) -> Result<f64, SimulationError> {
    if values.is_empty() {
        return Err(SimulationError::EmptyMoveArray);
    }

    let sum: u64 = values.iter().sum();
    Ok(sum as f64 / values.len() as f64)
}

pub fn calculate_variance(values: &[u64], mean: f64) -> Result<f64, SimulationError> {
    if values.is_empty() {
        return Err(SimulationError::EmptyMoveArray);
    }

    let sum_squares = values.iter().map(|&x| x.pow(2)).sum::<u64>() as f64;
    let count = values.len() as f64;
    Ok((sum_squares / count) - (mean * mean))
}

pub fn calculate_skewness(values: &[u64], mean: f64, std_dev: f64) -> Result<f64, SimulationError> {
    if values.is_empty() {
        return Err(SimulationError::EmptyMoveArray);
    }
    if std_dev == 0.0 {
        return Err(SimulationError::StatisticalError(
            "Standard deviation is zero when calculating skewness".to_string(),
        ));
    }

    let count = values.len() as f64;
    Ok(values
        .iter()
        .map(|&x| ((x as f64 - mean) / std_dev).powi(3))
        .sum::<f64>()
        / count)
}

pub fn calculate_kurtosis(values: &[u64], mean: f64, std_dev: f64) -> Result<f64, SimulationError> {
    if values.is_empty() {
        return Err(SimulationError::EmptyMoveArray);
    }
    if std_dev == 0.0 {
        return Err(SimulationError::StatisticalError(
            "Standard deviation is zero when calculating kurtosis".to_string(),
        ));
    }

    let count = values.len() as f64;
    Ok(values
        .iter()
        .map(|&x| ((x as f64 - mean) / std_dev).powi(4))
        .sum::<f64>()
        / count)
}

fn calculate_median(values: &[u64]) -> f64 {
    let mut values_copy = values.to_owned();
    values_copy.sort();
    let len = values_copy.len();
    if len % 2 == 0 {
        (values_copy[len / 2 - 1] + values_copy[len / 2]) as f64 / 2.0
    } else {
        values_copy[len / 2] as f64
    }
}
