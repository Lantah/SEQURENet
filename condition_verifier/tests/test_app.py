import unittest
import os
import tempfile
from app import app

class ConditionVerifierTestCase(unittest.TestCase):
    def setUp(self):
        self.app = app.test_client()
        self.app.testing = True
        self.test_file = tempfile.NamedTemporaryFile(delete=False)
        self.test_file.write(b"test data")
        self.test_file.close()

    def tearDown(self):
        os.unlink(self.test_file.name)
    
    def test_verify_valid_file(self):
        with open(self.test_file.name, "rb") as f:
            response = self.app.post("/verify", data={"file": f})
        self.assertEqual(response.status_code, 200)
        self.assertIn("result", response.json)
    
    def test_verify_missing_file(self):
        response = self.app.post("/verify", data={})
        self.assertEqual(response.status_code, 400)
        self.assertIn("error", response.json)
    
    def test_verify_invalid_file_format(self):
        with open(self.test_file.name, "rb") as f:
            response = self.app.post("/verify", data={"file": (f, "invalid.txt")})
        self.assertEqual(response.status_code, 200)
        self.assertIn("result", response.json)

if __name__ == "__main__":
    unittest.main()
