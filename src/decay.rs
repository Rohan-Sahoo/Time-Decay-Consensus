use chrono::{Utc, DateTime};

#[derive(Debug, Clone)]
pub enum DecayModel {
    Exponential { lambda: f64 },
    Linear { total_duration_secs: i64 },
    Stepped { step_secs: i64, decay_per_step: f64 },
}

impl DecayModel {
    pub fn calculate_weight(&self, vote_time: DateTime<Utc>) -> f64 {
        let now = Utc::now();
        let elapsed = now.signed_duration_since(vote_time).num_seconds().max(0) as f64;

        match self {
            DecayModel::Exponential { lambda } => (-(lambda * elapsed)).exp().max(0.1),
            DecayModel::Linear { total_duration_secs } => {
                let remaining = (*total_duration_secs as f64 - elapsed).max(0.0);
                (remaining / *total_duration_secs as f64).max(0.1)
            }
            DecayModel::Stepped { step_secs, decay_per_step } => {
                let steps = (elapsed / *step_secs as f64).floor();
                ((1.0 - steps * decay_per_step) as f64).max(0.1)

            }
        }
    }
}
