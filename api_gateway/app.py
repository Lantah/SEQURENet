from flask import Flask, request, jsonify
import requests
import os
from dotenv import load_dotenv

load_dotenv()

app = Flask(__name__)
CONDITION_VERIFIER_URL = os.getenv("CONDITION_VERIFIER_URL", "http://localhost:5000/verify")

@app.route("/submit", methods=["POST"])
def submit_request():
    if "file" not in request.files:
        return jsonify({"error": "No file uploaded"}), 400
    
    file = request.files["file"]
    files = {"file": (file.filename, file.stream, file.mimetype)}
    
    try:
        response = requests.post(CONDITION_VERIFIER_URL, files=files)
        return jsonify(response.json()), response.status_code
    except requests.RequestException as e:
        return jsonify({"error": "Failed to reach condition verifier", "details": str(e)}), 500

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8000)
