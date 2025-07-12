use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone)]
pub enum VotingWindowType {
    Short,
    Medium,
    Long,
    Custom(Duration),
}

#[derive(Debug, Clone)]
pub struct VotingWindow {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub grace_period: Duration,
    pub extended: bool,
}

impl VotingWindow {
    pub fn new(window_type: VotingWindowType, grace_secs: i64) -> Self {
        let now = Utc::now();
        let duration = match window_type {
            VotingWindowType::Short => Duration::minutes(5),
            VotingWindowType::Medium => Duration::minutes(30),
            VotingWindowType::Long => Duration::hours(2),
            VotingWindowType::Custom(d) => d,
        };

        VotingWindow {
            start_time: now,
            end_time: now + duration,
            grace_period: Duration::seconds(grace_secs),
            extended: false,
        }
    }

    pub fn has_expired(&self) -> bool {
        Utc::now() > self.end_time + self.grace_period
    }

    pub fn should_extend(&self, votes_needed: usize, votes_remaining: usize) -> bool {
        let nearing_end = (self.end_time - Utc::now()) <= Duration::minutes(1);
        let close_to_threshold = votes_remaining <= votes_needed;

        nearing_end && close_to_threshold && !self.extended
    }

    pub fn extend_window(&mut self, extension_duration: Duration) {
        if !self.extended {
            self.end_time += extension_duration;
            self.extended = true;
        }
    }

    pub fn is_active(&self) -> bool {
    !self.has_expired()
}


    pub fn overlaps(&self, other: &VotingWindow) -> bool {
        self.start_time < other.end_time && self.end_time > other.start_time
    }
}
