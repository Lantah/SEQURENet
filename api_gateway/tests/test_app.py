import unittest
import os
import tempfile
from app import app
from unittest.mock import patch
import requests

class APIGatewayTestCase(unittest.TestCase):
    def setUp(self):
        self.app = app.test_client()
        self.app.testing = True
        self.test_file = tempfile.NamedTemporaryFile(delete=False)
        self.test_file.write(b"test data")
        self.test_file.close()

    def tearDown(self):
        os.unlink(self.test_file.name)
    
    @patch("requests.post")
    def test_submit_valid_file(self, mock_post):
        mock_post.return_value.status_code = 200
        mock_post.return_value.json.return_value = {"result": True}
        
        with open(self.test_file.name, "rb") as f:
            response = self.app.post("/submit", data={"file": f})
        
        self.assertEqual(response.status_code, 200)
        self.assertIn("result", response.json)
    
    def test_submit_missing_file(self):
        response = self.app.post("/submit", data={})
        self.assertEqual(response.status_code, 400)
        self.assertIn("error", response.json)
    
    @patch("requests.post")
    def test_submit_verifier_unreachable(self, mock_post):
        mock_post.side_effect = requests.RequestException("Verifier unreachable")
        
        with open(self.test_file.name, "rb") as f:
            response = self.app.post("/submit", data={"file": f})
        
        self.assertEqual(response.status_code, 500)
        self.assertIn("error", response.json)

if __name__ == "__main__":
    unittest.main()