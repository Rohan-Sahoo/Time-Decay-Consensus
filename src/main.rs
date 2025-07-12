 // Time-Decay-Concensus
 
mod vote;
mod decay;
mod threshold;
mod voting_window;
mod validators;

use chrono::{Utc, Duration};
use vote::{Vote, VotePool};
use decay::DecayModel;
use threshold::EscalationModel;
use voting_window::{VotingWindow, VotingWindowType};
use validators::Validator;

fn main() {
    println!("Time-Decay Threshold Consensus Engine Starting...");

    let decay_model = DecayModel::Exponential { lambda: 0.001 };
    let threshold_model = EscalationModel::Linear { rate_per_sec: 0.001 };

    let validator = Validator::new("Validator_1".to_string(), true);

    let decay = DecayModel::Stepped { step_secs: 10, decay_per_step: 0.5 };
    let escalation = EscalationModel::Sigmoid { steepness: 1.2 };

    let mut window = VotingWindow::new(
        VotingWindowType::Custom(Duration::seconds(20)),
        5, // grace period
    );

    println!("Voting started at: {}", window.start_time);
    println!("Voting ends at:    {}", window.end_time);

    if window.has_expired() {
        println!("Voting window expired.");
    } else {
        println!("Voting window is still active.");
    }

    let vote = Vote {
        voter_id: "voter123".to_string(),
        signature: "fake_signature".to_string(),
        timestamp: Utc::now(),
        decay_model: DecayModel::Exponential { lambda: 0.5 },
        weight: 1.0,
    };

    println!("{:?}", vote);
    println!("Vote decay model: {:?}", vote.decay_model);
    println!("Escalation model: {:?}", escalation);

    let mut pool = VotePool::new();
    pool.display();

    let signatures = vec![
        "signed_data_placeholder",
        "signed_data_placeholder",
        "invalid_signature",
    ];

    let voter_ids = vec!["Voter_1", "Voter_2", "Voter_3"];

    for i in 0..voter_ids.len() {
        let vote = Vote::new(
            voter_ids[i].to_string(),
            decay_model.clone(),
            signatures[i].to_string(),
        );

        if validator.verify_signature(&vote.signature) && window.is_active() {
            pool.add_vote(vote);
        } else {
            println!("Vote from {} rejected (invalid signature or expired window)", voter_ids[i]);
        }
    }

    let threshold = threshold_model.calculate_threshold(window.start_time);
    let current_weight = pool.total_weight();

    println!("\nFinal Voting Result:");
    println!("Total Vote Weight: {:.4}", current_weight);
    println!("Required Threshold: {:.2}%", threshold * 100.0);

    if current_weight >= threshold {
        println!("Proposal PASSED consensus.");
    } else {
        println!("Proposal FAILED to reach consensus.");
    }
}

