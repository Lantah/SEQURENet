### README.md

# Overview

This project implements a **decentralized verification system** using an **SCP-style consensus mechanism** among pseudovalidators. It enables **off-chain validation** and **on-chain publication** of verification results.

## Workflow

1. **User Requests**: A user submits a verification request via an API gateway.
2. **Condition Verification**: A Python-based microservice processes the request (e.g., image validation) and returns a boolean result.
3. **Pseudovalidator Voting**: Rust-based pseudovalidators verify the condition, sign their votes, and exchange them via a messaging layer.
4. **Distributed Consensus**: Once a quorum is reached, votes are aggregated off-chain.
5. **On-Chain Publication**: Aggregated results are submitted to a Rust-based smart contract for verification and storage on-chain.

---

## Repository Structure

```
your-project/
├── README.md                # Overview and quickstart guide
├── docs/                    # Documentation
│   ├── architecture.md      # System design, flowcharts, module interactions
│   ├── setup.md             # Setup instructions
│   └── future_features.md   # Future enhancements & changelog
├── smart_contract/          # Rust-based smart contract
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs           # Vote aggregation & on-chain publishing logic
│   └── tests/               # Smart contract tests
├── pseudovalidator/         # Off-chain Rust pseudovalidator logic
│   ├── config/
│   │   └── config.yaml      # Validator ID, keys, trusted validators, etc.
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── config.rs        # Configuration loader
│   │   ├── networking.rs    # Peer-to-peer communication
│   │   ├── quorum.rs        # Quorum verification logic
│   │   └── crypto.rs        # Signing & verification
│   └── tests/               # Unit & integration tests
├── condition_verifier/      # Python microservice for verification
│   ├── app.py               # Flask service handling validation requests
│   ├── requirements.txt     # Python dependencies
│   └── tests/               # Unit tests
├── api_gateway/             # (Optional) Python API Gateway
│   ├── app.py               # Flask-based API
│   ├── requirements.txt     # Dependencies
│   └── tests/               # Unit tests
├── messaging/               # (Optional) Messaging infrastructure (RabbitMQ/Kafka)
├── docker-compose.yml       # Orchestrates all services
├── .env                     # Environment variables
├── .gitignore               # Ignore patterns
└── CI/                      # CI/CD configurations
```

---

## Getting Started

### Prerequisites

Ensure you have the following installed:

- **Rust** (latest stable version)
- **Python** (3.8+)
- **Docker & Docker Compose**
- **PostgreSQL** (if using a database for logging or future enhancements)

### Installation

#### 1. Clone the Repository

```sh
git clone https://github.com/your-org/your-project.git
cd your-project
```

#### 2. Setup Environment

Create a **.env** file with necessary environment variables.

```sh
# Example .env file
NETWORK_HOST=127.0.0.1
API_PORT=5000
POSTGRES_URL=postgresql://user:password@localhost/db_name
```

#### 3. Install Dependencies

**Rust Dependencies**
```sh
cd pseudovalidator/
cargo build --release
```

**Python Dependencies**
```sh
cd condition_verifier/
pip install -r requirements.txt
```

#### 4. Run Services

**Using Docker Compose**:
```sh
docker-compose up --build
```

**Manually**:
```sh
# Start Condition Verifier
cd condition_verifier/
python app.py

# Start Pseudovalidator
cd pseudovalidator/
cargo run --release
```

---

## Testing

Run tests for each component:

#### **Smart Contract**
```sh
cd smart_contract/
cargo test
```

#### **Pseudovalidator**
```sh
cd pseudovalidator/
cargo test
```

#### **Condition Verifier**
```sh
cd condition_verifier/
pytest
```

---

## Architecture

The system is composed of the following components:

- **Rust Smart Contract**: Handles vote verification and on-chain publishing.
- **Rust Pseudovalidator**: Manages voting and quorum logic.
- **Python Condition Verifier**: Processes verification requests.
- **(Optional) API Gateway**: Simplifies user interaction.
- **(Optional) Messaging Layer**: Facilitates inter-validator communication.

For more details, see **docs/architecture.md**.

---

## Future Enhancements

Planned improvements include:

- Advanced AI-based verification models.
- Multi-chain support for verification results.
- Decentralized messaging via **libp2p**.
- Enhanced security mechanisms for validator identity management.

---

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -m "Description of changes"`).
4. Push to your branch (`git push origin feature-branch`).
5. Submit a **pull request**.

---

## License

This project is licensed under the **MIT License**.

