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
    min_moves_fraction: f64,
    mean: f64,
    variance: f64,
    std_deviation: f64,
    coeff_variation: f64,
    skewness: f64,
    kurtosis: f64,
}

impl SimulationStats {
    /// Calculate statistical metrics from a slice of move counts
    pub(crate) fn from_moves(moves: &[u64]) -> Result<Self, SimulationError> {
        if moves.is_empty() {
            return Err(SimulationError::EmptyMoveArray);
        }

        let min_moves = *moves.iter().min().unwrap(); // Safe because we checked for empty
        let min_moves_count = moves.iter().filter(|&m| m == &min_moves).count();
        let count = moves.len() as f64;

        let mean = calculate_mean(moves)?;
        let variance = calculate_variance(moves, mean)?;
        let std_deviation = variance.sqrt();

        if std_deviation == 0.0 {
            return Err(SimulationError::StatisticalError(
                "Standard deviation is zero".to_string()
            ));
        }

        Ok(SimulationStats {
            min_moves,
            min_moves_fraction: (min_moves_count as f64) / count,
            mean,
            variance,
            std_deviation,
            coeff_variation: std_deviation / mean,
            skewness: calculate_skewness(moves, mean, std_deviation)?,
            kurtosis: calculate_kurtosis(moves, mean, std_deviation)?,
        })
    }

    /// Print all statistical metrics
    pub(crate) fn print(&self) {
        println!("Shortest Path Length: {}", self.min_moves);
        println!("Shortest Path Fraction: {}", self.min_moves_fraction);
        println!("Mean moves: {}", self.mean);
        println!("Variance: {}", self.variance);
        println!("Standard deviation: {}", self.std_deviation);
        println!("Coefficient of Variation: {}", self.coeff_variation);
        println!("Skew: {}", self.skewness);
        println!("Kurtosis: {}", self.kurtosis);
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

    let sum_squares = values.iter()
        .map(|&x| x.pow(2))
        .sum::<u64>() as f64;
    let count = values.len() as f64;
    Ok((sum_squares / count) - (mean * mean))
}

pub fn calculate_skewness(values: &[u64], mean: f64, std_dev: f64) -> Result<f64, SimulationError> {
    if values.is_empty() {
        return Err(SimulationError::EmptyMoveArray);
    }
    if std_dev == 0.0 {
        return Err(SimulationError::StatisticalError(
            "Standard deviation is zero when calculating skewness".to_string()
        ));
    }

    let count = values.len() as f64;
    Ok(values.iter()
        .map(|&x| ((x as f64 - mean) / std_dev).powi(3))
        .sum::<f64>() / count)
}

pub fn calculate_kurtosis(values: &[u64], mean: f64, std_dev: f64) -> Result<f64, SimulationError> {
    if values.is_empty() {
        return Err(SimulationError::EmptyMoveArray);
    }
    if std_dev == 0.0 {
        return Err(SimulationError::StatisticalError(
            "Standard deviation is zero when calculating kurtosis".to_string()
        ));
    }

    let count = values.len() as f64;
    Ok(values.iter()
        .map(|&x| ((x as f64 - mean) / std_dev).powi(4))
        .sum::<f64>() / count)
}