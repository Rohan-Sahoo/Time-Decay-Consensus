use crate::decay::DecayModel;
use chrono::{Utc, DateTime};

#[derive(Debug, Clone)]
pub struct Vote {
    pub voter_id: String,
    pub timestamp: DateTime<Utc>,
    pub weight: f64,
    pub decay_model: DecayModel,
    pub signature: String,
}

impl Vote {
    pub fn new(voter_id: String, decay_model: DecayModel, signature: String) -> Self {
        let timestamp = Utc::now();
        let weight = decay_model.calculate_weight(timestamp);
        Vote {
            voter_id,
            timestamp,
            weight,
            decay_model,
            signature,
        }
    }
}

#[derive(Debug)]
pub struct VotePool {
    pub votes: Vec<Vote>,
}

impl VotePool {
    pub fn new() -> Self {
        VotePool {
            votes: Vec::new(),
        }
    }

    pub fn add_vote(&mut self, vote: Vote) {
        self.votes.push(vote);
        println!("Vote added to pool");
    }

    pub fn total_weight(&self) -> f64 {
        self.votes.iter().map(|v| v.weight).sum::<f64>().max(0.1)
    }

    pub fn display(&self) {
    println!("\n📜 Vote Pool:");
    for (i, vote) in self.votes.iter().enumerate() {
        println!(
            "Vote {} - Voter: {}, Weight: {:.4}, Time: {}",
            i + 1,
            vote.voter_id,
            vote.weight,
            vote.timestamp
        );
    }
}

}
