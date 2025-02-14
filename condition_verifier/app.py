from flask import Flask, request, jsonify
import subprocess
import yaml
import os

app = Flask(__name__)

# Load configuration
CONFIG_PATH = "config.yaml"
if os.path.exists(CONFIG_PATH):
    with open(CONFIG_PATH, "r") as file:
        config = yaml.safe_load(file)
else:
    config = {}

EXTERNAL_VERIFIER = config.get("external_verifier", "./external_verifier")

def call_external_verifier(file_path):
    """Calls an external verification program and returns its result."""
    try:
        result = subprocess.run([EXTERNAL_VERIFIER, file_path], capture_output=True, text=True)
        return result.stdout.strip().lower() == "true"
    except Exception as e:
        return False


@app.route("/verify", methods=["POST"])
def verify():
    if "file" not in request.files:
        return jsonify({"error": "No file uploaded"}), 400
    
    file = request.files["file"]
    file_path = f"/tmp/{file.filename}"
    file.save(file_path)
    
    result = call_external_verifier(file_path)
    os.remove(file_path)
    
    return jsonify({"result": result})


if __name__ == "__main__":
    app.run(host=config.get("host", "0.0.0.0"), port=config.get("port", 5000))
