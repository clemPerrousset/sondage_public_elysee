import requests
import time
import json

BASE_URL = "http://localhost:3000"
ADMIN_KEY = "admin_secret_key_99999"

def test_flow():
    print("Starting tests (Play Integrity / Apple Check)...")
    
    # 1. Vote for "Alice" with device1 (Android)
    print("\n1. Voting for Alice (device1 - Android)...")
    resp = requests.post(
        f"{BASE_URL}/vote",
        json={
            "device_id": "device1", 
            "candidate_name": "Alice",
            "os": "android",
            "token": "mock_android_token"
        }
    )
    print(f"Status: {resp.status_code}")
    if resp.status_code != 200:
        print(resp.text)
    assert resp.status_code == 200

    # 2. Vote for "Bob" with device2 (iOS)
    print("\n2. Voting for Bob (device2 - iOS)...")
    resp = requests.post(
        f"{BASE_URL}/vote",
        json={
            "device_id": "device2", 
            "candidate_name": "Bob",
            "os": "ios",
            "token": "mock_ios_token"
        }
    )
    print(f"Status: {resp.status_code}")
    assert resp.status_code == 200

    # 3. Invalid Token Test
    print("\n3. Testing Invalid Token...")
    resp = requests.post(
        f"{BASE_URL}/vote",
        json={
            "device_id": "device3", 
            "candidate_name": "Charlie",
            "os": "android",
            "token": "invalid_token"
        }
    )
    print(f"Status: {resp.status_code}")
    assert resp.status_code == 401

    # 4. Check percentages
    print("\n4. Checking percentages...")
    resp = requests.get(f"{BASE_URL}/percentage")
    print(f"Response: {resp.json()}")
    data = resp.json()
    assert len(data) == 2

    # 5. Delete "Alice" (Admin)
    print("\n5. Deleting Alice (Admin)...")
    resp = requests.delete(
        f"{BASE_URL}/candidate",
        headers={"X-Admin-Key": ADMIN_KEY, "Content-Type": "application/json"},
        json={"name": "Alice"}
    )
    print(f"Status: {resp.status_code}")
    assert resp.status_code == 200

    print("\nAll tests passed!")

if __name__ == "__main__":
    try:
        test_flow()
    except Exception as e:
        print(f"Test failed: {e}")
        exit(1)
