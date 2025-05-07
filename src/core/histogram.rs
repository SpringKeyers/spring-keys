use std::collections::VecDeque;
use std::time::{Instant, Duration};
use serde::{Serialize, Deserialize};

/// Represents a range in the histogram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramRange {
    pub start: f64,
    pub end: f64,
    pub count: usize,
    pub samples: Vec<f64>,
}

/// Statistics for histogram data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramStats {
    pub min: f64,
    pub max: f64,
    pub bucket_size: f64,
    pub buckets: Vec<usize>,
    pub total_samples: usize,
    pub running_geo_avg: f64,
    pub total_product: f64,
    pub total_count: usize,
    #[serde(skip)]
    pub times_10s: VecDeque<(Instant, f64)>,
    #[serde(skip)]
    pub times_60s: VecDeque<(Instant, f64)>,
    pub running_10s_avg: f64,
    pub running_60s_avg: f64,
    pub ranges: Vec<HistogramRange>,
}

impl HistogramStats {
    pub fn new() -> Self {
        Self {
            min: f64::INFINITY,
            max: 0.0,
            bucket_size: 0.0,
            buckets: Vec::new(),
            total_samples: 0,
            running_geo_avg: 0.0,
            total_product: 1.0,
            total_count: 0,
            times_10s: VecDeque::new(),
            times_60s: VecDeque::new(),
            running_10s_avg: 0.0,
            running_60s_avg: 0.0,
            ranges: Vec::new(),
        }
    }

    pub fn new_key_speed() -> Self {
        Self {
            min: 0.0,
            max: 1000.0,
            bucket_size: 50.0,
            buckets: vec![0; 20],
            total_samples: 0,
            running_geo_avg: 0.0,
            total_product: 1.0,
            total_count: 0,
            times_10s: VecDeque::new(),
            times_60s: VecDeque::new(),
            running_10s_avg: 0.0,
            running_60s_avg: 0.0,
            ranges: Vec::new(),
        }
    }

    pub fn new_wpm() -> Self {
        Self {
            min: 0.0,
            max: 200.0,
            bucket_size: 10.0,
            buckets: vec![0; 20],
            total_samples: 0,
            running_geo_avg: 0.0,
            total_product: 1.0,
            total_count: 0,
            times_10s: VecDeque::new(),
            times_60s: VecDeque::new(),
            running_10s_avg: 0.0,
            running_60s_avg: 0.0,
            ranges: Vec::new(),
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
    pub fn add_value(&mut self, value: f64) {
        if value <= 0.0 || value.is_nan() || value.is_infinite() {
            return;
        }

        let now = Instant::now();

        // Update min/max values
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        
        // Update session-wide arithmetic mean
        self.total_count += 1;
        let prev_mean = self.running_geo_avg;
        self.running_geo_avg = self.total_product.powf(1.0 / self.total_count as f64);
        
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

        // Find the appropriate bucket for this value
        let bucket_index = ((value - self.min) / self.bucket_size) as usize;
        if bucket_index < self.buckets.len() {
            self.buckets[bucket_index] += 1;
            self.total_samples += 1;
        }
    }

    /// Reset current quote statistics while maintaining session data
    pub fn reset_current(&mut self) {
        self.buckets.fill(0);
        for range in &mut self.ranges {
            range.count = 0;
            range.samples.clear();
        }
        self.times_10s.clear();
        self.times_60s.clear();
        self.running_10s_avg = 0.0;
        self.running_60s_avg = 0.0;
    }

    pub fn get_mean(&self) -> f64 {
        if self.total_samples > 0 {
            self.running_geo_avg
        } else {
            0.0
        }
    }
} 