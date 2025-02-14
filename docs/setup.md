Setup Guide

## Prerequisites

Ensure you have the following installed on your system:

- Rust (latest stable version)
- Python 3.8+
- Docker & Docker Compose
- PostgreSQL (if using a database for logging or future enhancements)
- External Verification Program (must be installed and configured)

## Installation Steps

### 1. Clone the Repository

```sh
git clone https://github.com/your-org/your-project.git
cd your-project
```

### 2. Set Up Environment Variables

Create a `.env` file in the root directory with necessary environment variables:

```sh
# Example .env file
NETWORK_HOST=127.0.0.1
API_PORT=5000
POSTGRES_URL=postgresql://user:password@localhost/db_name
EXTERNAL_VERIFIER=./path/to/your/verifier
```

### 3. Install Dependencies

#### Rust Dependencies

```sh
cd pseudovalidator/
cargo build --release
```

#### Python Dependencies

```sh
cd condition_verifier/
pip install -r requirements.txt
```

### 4. Start the Services

#### Using Docker Compose:

```sh
docker-compose up --build
```

#### Or manually:

##### Start Condition Verifier

```sh
cd condition_verifier/
python app.py
```

##### Start Pseudovalidator

```sh
cd pseudovalidator/
cargo run --release
```

### 5. Run Tests

#### Smart Contract Tests

```sh
cd smart_contract/
cargo test
```

#### Pseudovalidator Tests

```sh
cd pseudovalidator/
cargo test
```

#### Condition Verifier Tests

```sh
cd condition_verifier/
pytest
```

### 6. Deploy Smart Contract (if applicable)

If deployment is needed, refer to the blockchain documentation.

```sh
cd smart_contract/
cargo build --release
```

### 7. Verify Deployment

Ensure all services are running correctly and interacting as expected.

```sh
curl -X POST http://localhost:5000/verify -F "file=@sample_input_file"
```

