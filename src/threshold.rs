use chrono::{Utc, DateTime};

#[derive(Debug, Clone)]
pub enum EscalationModel {
    Linear { rate_per_sec: f64 },
    Exponential { base: f64 },
    Sigmoid { steepness: f64 },
}

impl EscalationModel {
    pub fn calculate_threshold(&self, start_time: DateTime<Utc>) -> f64 {
        let now = Utc::now();
        let elapsed = now.signed_duration_since(start_time).num_seconds().max(0) as f64;

        let threshold = match self {
            EscalationModel::Linear { rate_per_sec } => 0.51 + rate_per_sec * elapsed,
            EscalationModel::Exponential { base } => 0.51 + (base.powf(elapsed / 60.0) - 1.0) / 100.0,
            EscalationModel::Sigmoid { steepness } => {
                let x = elapsed / 60.0;
                0.51 + (1.0 / (1.0 + (-steepness * (x - 5.0)).exp()))
            }
        };

        threshold.clamp(0.51, 0.90)
    }
}
