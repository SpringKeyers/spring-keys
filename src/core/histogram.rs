use std::collections::VecDeque;
use std::time::Instant;

/// Represents a range in the histogram
#[derive(Debug, Clone, Copy)]
pub struct HistogramRange {
    pub min: f64,
    pub max: f64,
}

/// Statistics for histogram data
#[derive(Debug, Clone)]
pub struct HistogramStats {
    // Accumulated data for entire session
    pub total_distribution: Vec<usize>,
    // Current quote/typing session
    pub current_distribution: Vec<usize>,
    // Rolling averages
    pub avg_10s_distribution: Vec<usize>,
    pub avg_60s_distribution: Vec<usize>,
    // Statistical measures
    pub geometric_mean: f64,
    pub arithmetic_mean: f64,
    pub min_value: f64,
    pub max_value: f64,
    // Running averages for the entire session
    pub running_10s_avg: f64,
    pub running_60s_avg: f64,
    pub running_geo_avg: f64,
    // Timing data for session-wide rolling averages
    pub times_10s: VecDeque<(Instant, f64)>,
    pub times_60s: VecDeque<(Instant, f64)>,
    // Quote-specific averages
    pub quote_10s_avg: f64,
    pub quote_60s_avg: f64,
    pub quote_geo_avg: f64,
    quote_times_10s: VecDeque<(Instant, f64)>,
    quote_times_60s: VecDeque<(Instant, f64)>,
    quote_product: f64,
    quote_count: usize,
    // Total values for session-wide geometric mean
    total_product: f64,
    total_count: usize,
    // Range definitions
    pub ranges: Vec<HistogramRange>,
}

impl HistogramStats {
    /// Create a new histogram for key speed tracking (ms)
    pub fn new_key_speed() -> Self {
        let ranges = vec![
            HistogramRange { min: 0.0, max: 50.0 },
            HistogramRange { min: 50.0, max: 100.0 },
            HistogramRange { min: 100.0, max: 150.0 },
            HistogramRange { min: 150.0, max: 200.0 },
            HistogramRange { min: 200.0, max: 250.0 },
            HistogramRange { min: 250.0, max: 300.0 },
            HistogramRange { min: 300.0, max: 350.0 },
            HistogramRange { min: 350.0, max: 400.0 },
            HistogramRange { min: 400.0, max: 450.0 },
            HistogramRange { min: 450.0, max: f64::INFINITY },
        ];
        Self::new(ranges)
    }

    /// Create a new histogram for WPM tracking
    pub fn new_wpm() -> Self {
        let ranges = vec![
            HistogramRange { min: 0.0, max: 20.0 },
            HistogramRange { min: 20.0, max: 40.0 },
            HistogramRange { min: 40.0, max: 60.0 },
            HistogramRange { min: 60.0, max: 80.0 },
            HistogramRange { min: 80.0, max: 100.0 },
            HistogramRange { min: 100.0, max: 120.0 },
            HistogramRange { min: 120.0, max: 140.0 },
            HistogramRange { min: 140.0, max: 160.0 },
            HistogramRange { min: 160.0, max: 180.0 },
            HistogramRange { min: 180.0, max: f64::INFINITY },
        ];
        Self::new(ranges)
    }

    /// Create a new histogram with specified ranges
    fn new(ranges: Vec<HistogramRange>) -> Self {
        let bucket_count = ranges.len();
        Self {
            total_distribution: vec![0; bucket_count],
            current_distribution: vec![0; bucket_count],
            avg_10s_distribution: vec![0; bucket_count],
            avg_60s_distribution: vec![0; bucket_count],
            geometric_mean: 0.0,
            arithmetic_mean: 0.0,
            min_value: f64::INFINITY,
            max_value: 0.0,
            running_10s_avg: 0.0,
            running_60s_avg: 0.0,
            running_geo_avg: 0.0,
            times_10s: VecDeque::new(),
            times_60s: VecDeque::new(),
            quote_10s_avg: 0.0,
            quote_60s_avg: 0.0,
            quote_geo_avg: 0.0,
            quote_times_10s: VecDeque::new(),
            quote_times_60s: VecDeque::new(),
            quote_product: 1.0,
            quote_count: 0,
            total_product: 1.0,
            total_count: 0,
            ranges,
        }
    }

    /// Convert milliseconds per character to WPM
    pub fn ms_to_wpm(ms_per_char: f64) -> f64 {
        // 60000ms/min * (1char/ms_per_char) * (1word/5chars)
        60000.0 / (ms_per_char * 5.0)
    }

    /// Convert WPM to milliseconds per character
    pub fn wpm_to_ms(wpm: f64) -> f64 {
        // (60000ms/min) / (wpm * 5chars/word)
        60000.0 / (wpm * 5.0)
    }

    /// Record a new value in the histogram
    pub fn record_value(&mut self, value: f64) {
        if value <= 0.0 || value.is_nan() || value.is_infinite() {
            return;
        }

        let now = Instant::now();

        // Update min/max values
        self.min_value = self.min_value.min(value);
        self.max_value = self.max_value.max(value);
        
        // Update session-wide arithmetic mean
        self.total_count += 1;
        let prev_mean = self.arithmetic_mean;
        self.arithmetic_mean = prev_mean + (value - prev_mean) / self.total_count as f64;
        
        // Update session-wide geometric mean
        self.total_product *= value;
        self.running_geo_avg = self.total_product.powf(1.0 / self.total_count as f64);
        
        // Update quote-specific geometric mean
        self.quote_count += 1;
        self.quote_product *= value;
        self.quote_geo_avg = self.quote_product.powf(1.0 / self.quote_count as f64);
        
        // Update session-wide running averages
        self.times_10s.push_back((now, value));
        while let Some((time, _)) = self.times_10s.front() {
            if now.duration_since(*time).as_secs() > 10 {
                self.times_10s.pop_front();
            } else {
                break;
            }
        }
        if !self.times_10s.is_empty() {
            self.running_10s_avg = self.times_10s.iter()
                .map(|(_, v)| v)
                .sum::<f64>() / self.times_10s.len() as f64;
        }
        
        self.times_60s.push_back((now, value));
        while let Some((time, _)) = self.times_60s.front() {
            if now.duration_since(*time).as_secs() > 60 {
                self.times_60s.pop_front();
            } else {
                break;
            }
        }
        if !self.times_60s.is_empty() {
            self.running_60s_avg = self.times_60s.iter()
                .map(|(_, v)| v)
                .sum::<f64>() / self.times_60s.len() as f64;
        }

        // Update quote-specific running averages
        self.quote_times_10s.push_back((now, value));
        while let Some((time, _)) = self.quote_times_10s.front() {
            if now.duration_since(*time).as_secs() > 10 {
                self.quote_times_10s.pop_front();
            } else {
                break;
            }
        }
        if !self.quote_times_10s.is_empty() {
            self.quote_10s_avg = self.quote_times_10s.iter()
                .map(|(_, v)| v)
                .sum::<f64>() / self.quote_times_10s.len() as f64;
        }
        
        self.quote_times_60s.push_back((now, value));
        while let Some((time, _)) = self.quote_times_60s.front() {
            if now.duration_since(*time).as_secs() > 60 {
                self.quote_times_60s.pop_front();
            } else {
                break;
            }
        }
        if !self.quote_times_60s.is_empty() {
            self.quote_60s_avg = self.quote_times_60s.iter()
                .map(|(_, v)| v)
                .sum::<f64>() / self.quote_times_60s.len() as f64;
        }
        
        // Find the appropriate bucket for this value
        for (i, range) in self.ranges.iter().enumerate() {
            if value >= range.min && (value < range.max || (i == self.ranges.len() - 1 && value >= range.max)) {
                self.total_distribution[i] += 1;
                self.current_distribution[i] += 1;
                break;
            }
        }
    }

    /// Reset current quote statistics while maintaining session data
    pub fn reset_current(&mut self) {
        // Reset quote-specific stats
        self.current_distribution.fill(0);
        self.quote_10s_avg = 0.0;
        self.quote_60s_avg = 0.0;
        self.quote_geo_avg = 0.0;
        self.quote_times_10s.clear();
        self.quote_times_60s.clear();
        self.quote_product = 1.0;
        self.quote_count = 0;
        
        // Keep session-wide stats intact
        // (total_product, total_count, running averages, etc.)
    }
} 