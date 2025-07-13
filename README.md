# ⏳🗳️ Time-Decay Threshold Consensus (Rust)

A modular Rust implementation of a **Time-Decay Voting Consensus Algorithm**, where vote influence decreases over time. Built to simulate decentralized governance systems with configurable decay, thresholds, validator signatures, and voting windows.

> 

---

## 📌 Key Features

- 🕒 **Time-Weighted Voting**
  - Vote weight reduces over time to favor early participation.
- 📉 **Multiple Decay Models**
  - `Exponential`: Early votes dominate
  - `Linear`: Gradual influence drop
  - `Stepped`: Fixed drops after time intervals
- 🔐 **Validator Signature Verification**
  - Each vote must be signed using a trusted validator key.
- 📊 **Dynamic Threshold Escalation**
  - Voting threshold increases over time to reflect consensus urgency.
- ⏰ **Voting Window Control**
  - Only votes within a valid time window are counted.

---

## 🏗️ Project Structure

[Vote] → [Timestamp] → [Decay Model] → [Weighted Vote]
                               ↓
                       [Threshold Model]
                               ↓
                   [Voting Window + Signature]
                               ↓
                          [Consensus Engine]

📚 Use Cases
Blockchain governance systems

DAO voting mechanisms

Real-time consensus in distributed environments

