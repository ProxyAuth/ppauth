
<div align="center">
<h1>PyProxyAuth</h1>
<img src='https://github.com/ProxyAuth/ppauth/raw/main/images/ppauth.png' width="300px" height="300px"/>
</div>
<br>
A lightweight Python library to authenticate and retrieve tokens via ProxyAuth.  

## install 
```
pip install ppauth
```

## usage

```python
import ppauth

ppauth.auth(
    host="127.0.0.1", port=8080,
    username="admin", password="admin123",
    timezone="Europe/Paris"
)

token = ppauth.token()
token = ppauth.token(renew=True) # Automatically re-authenticates and returns a new token if the previous one has expired.
lease_token = ppauth.lease_token()

print({"token": token, "expire_at": lease_token})

# result
# {"expire_at": 16500,"token":"ZoHAauGmCyxjq6+1sfVbqy..."}

# for easy use method GET
# Use the route defined in routes.yml within your backend delivery. 
# You don't need to manually include the token in the headers it's handled automatically
headers = {"user-agent": "ppauth/0.1.1"}
params = {"params_key": "my_send_request_params_via_get"}

ppauth.get(
    "/app",          # The API route (relative path) to call on the authenticated backend
    headers=headers, # Optional custom HTTP headers (Python dict), e.g. {"X-User": "admin"}
    params=params,   # Optional query parameters (Python dict), e.g. {"limit": "10"}
    verify=False,    # Disable TLS certificate verification (useful for self-signed certs)
    timeout=5,       # Max duration (in seconds) to wait for a response before failing
    retry=5          # Number of retry attempts if the request fails (timeout or server error)
)

# result
# 'ok'

# you are similar for POST method
headers = {"user-agent": "ppauth/0.1.1"}
body = {"body_key": "my_send_request_data_via_post"}
json = {"json_key": "my_send_request_json_data_via_post"}

ppauth.post(
    "/app",          # The API route (relative path) to call on the authenticated backend
    headers=headers, # Optional custom HTTP headers (Python dict), e.g. {"Content-Type": "application/json"}
    body=body,       # (If implemented) Optional raw body content (usually a string or bytes)
    json=json,       # Optional JSON body (Python dict) that will be serialized and sent as application/json
    timeout=5,       # Max duration (in seconds) to wait for a response before raising a timeout error
    verify=False,    # Disable TLS certificate verification (useful for self-signed certificates)
    retry=2          # Number of retry attempts if the request fails (e.g., timeout or 5xx server errors)
)

# result
# 'ok'
```
