
<div align="center">
<h1>PyProxyAuth</h1>
<img src='images/ppauth.png' width="300px" height="300px"/>
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

> {"expire_at": 16500,"token":"ZoHAauGmCyxjq6+1sfVbqy..."}

# for easy use method GET
# Use the route defined in routes.yml within your backend delivery. 
# You don't need to manually include the token in the headers it's handled automatically
headers = {"user-agent": "ppauth/0.1.1"}
params = {"params_key": "my_send_request_params_via_get"}
ppauth.get("/app", headers=headers, params=params)

# result
> 'ok'

# you are similar for POST method
headers = {"user-agent": "ppauth/0.1.1"}
body = {"body_key": "my_send_request_data_via_post"}
ppauth.post("/app", headers, body)

# result
> 'ok'
```
