
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
```
