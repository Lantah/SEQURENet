Architecture Overview

## Introduction

The decentralized verification system is designed to facilitate trustless validation of user-submitted data using an SCP-style consensus mechanism among pseudovalidators. This architecture enables off-chain verification and ensures secure, efficient publication of verification results on-chain.

## System Components

### 1. API Gateway (Optional)

**Technology:** Python (Flask)

**Purpose:** Accepts user verification requests and forwards them to the pseudovalidator network.

**Responsibilities:**
- Receive user inputs (e.g., image, document, metadata).
- Validate request structure.
- Forward the request to a designated pseudovalidator.

### 2. Condition Verifier (Python Microservice)

**Technology:** Python (Flask, OpenCV, TensorFlow/PyTorch as applicable)

**Purpose:** Processes the submitted data to determine if it meets verification criteria.

**Responsibilities:**
- Accept HTTP verification requests.
- Call an external verification program to process the file.
- Return a boolean validation result (true/false).

### 3. Pseudovalidator Network (Rust)

**Technology:** Rust (Tokio, TCP Sockets, ed25519-dalek for cryptographic operations)

**Purpose:** Performs decentralized voting to achieve consensus on verification outcomes.

**Responsibilities:**
- Load configuration settings (validator ID, keys, quorum thresholds).
- Communicate with other pseudovalidators using **raw TCP messaging**.
- Sign verification results and exchange votes.
- Reach quorum using SCP (Stellar Consensus Protocol).
- Submit the final aggregated result to the blockchain.

### 4. Blockchain Smart Contract (Rust)

**Technology:** Rust (for custom Metriq Network smart contracts)

**Purpose:** Stores and validates the final verification results based on consensus.

**Responsibilities:**
- Accept aggregated votes from the pseudovalidator network.
- Validate digital signatures and ensure quorum requirements are met.
- Publish verified results on-chain.

### 5. Networking Layer (TCP-based P2P Communication)

**Technology:** Rust (Tokio, TCP Sockets)

**Purpose:** Facilitates direct peer-to-peer communication between pseudovalidators for vote sharing.

**Responsibilities:**
- Provide direct **TCP messaging** between pseudovalidators.
- Allow nodes to dynamically **discover and register peers**.
- Ensure reliable message delivery for quorum consensus.

## Workflow

### **User Submission:**
A user submits data for verification via the API Gateway (or directly to a pseudovalidator).

### **Condition Verification:**
The condition verifier forwards the request to an external verification program and receives a validation result.

### **Pseudovalidator Voting:**
- Each pseudovalidator queries the condition verifier independently.
- They sign their results and exchange votes using **direct TCP messaging**.
- Using SCP, they determine when a quorum has been reached.

### **On-Chain Publication:**
- If quorum is achieved, the aggregated votes are submitted to the smart contract.
- The blockchain verifies the signatures and records the result.

## Data Flow Diagram

[User] → [API Gateway] → [Condition Verifier] → [External Verifier] → [Pseudovalidators] → [Blockchain]

Pseudovalidators communicate **directly over TCP** instead of using a message broker.

## Security Considerations

- **Cryptographic Signatures:** Votes are signed using ed25519 to ensure authenticity.
- **Quorum Mechanism:** SCP ensures that a reliable consensus is reached before data is recorded on-chain.
- **Data Privacy:** No sensitive user data is stored on-chain; only verification outcomes.

## Future Enhancements

- Improved peer discovery and fault tolerance for TCP networking.
- Multi-chain interoperability.

For setup instructions, refer to `docs/setup.md`.
