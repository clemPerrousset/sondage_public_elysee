# Security Protocol: Play Integrity & Apple DeviceCheck

We are replacing the HMAC system with OS-native integrity checks.

## Overview
The app must obtain an integrity token from the OS and send it to the backend.

## 1. Android (Google Play Integrity)
- **App Side**:
    1. Call Standard Integrity API (or Classic).
    2. Obtain `token`.
- **Backend Side**:
    1. Receive `token`.
    2. Call `https://playintegrity.googleapis.com/v1/...:decodeIntegrityToken`.
    3. Verify `appLicensingVerdict` == `LICENSED` and `appRecognitionVerdict` == `RECOGNIZED_VERSION`.

## 2. iOS (Apple DeviceCheck / App Attest)
- **App Side**:
    1. Use `DCDevice` to generate a token.
- **Backend Side**:
    1. Receive `token`.
    2. Generate a JWT signed with your Apple Developer Key (`.p8`).
    3. Call `https://api.devicecheck.apple.com/v1/validate_device_token`.
    4. Verify response (200 OK).

## API Changes
### `POST /vote`
**Body**:
```json
{
  "device_id": "unique_uuid_generated_by_app",
  "candidate_name": "Alice",
  "os": "android", // or "ios"
  "token": "base64_encoded_token_from_os"
}
```

## Environment Variables Needed
```
# Google
GOOGLE_PACKAGE_NAME=com.example.app
GOOGLE_SERVICE_ACCOUNT_JSON=path/to/service_account.json

# Apple
APPLE_KEY_ID=ABC1234567
APPLE_TEAM_ID=DEF1234567
APPLE_P8_FILE_CONTENT=...
```
