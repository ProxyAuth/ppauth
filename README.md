# pyproxyauth 

A lightweight Python library to authenticate and retrieve tokens via ProxyAuth.  

## install 
```
pip install pyproxyauth
```

## usage

```
import pyproxyauth

pyproxyauth.auth(
    host="127.0.0.1", port=8080,
    username="admin", password="admin123"
    timezone="Europe/Paris"
)

token = pyproxyauth.token()
token = pyproxyauth.token(renew=True) # Automatically re-authenticates and returns a new token if the previous one has expired.
lease_token = pyproxyauth.lease_token()

print({"token": token, "expire_at": lease_token})

> {"expire_at": 16500,"token":"ZoHAauGmCyxjq6+1sfVbqy..."}
```
