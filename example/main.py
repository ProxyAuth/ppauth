import pyproxyauth

pyproxyauth.auth(
    host="127.0.0.1", port=8080,
    username="admin", password="admin123",
    timezone="Europe/Paris"
)

token = pyproxyauth.token()
# or use renew method
token = pyproxyauth.token(renew=True) # Automatically re-authenticates and returns a new token if the previous one has expired.
lease_token = pyproxyauth.lease_token()

print({"token": token, "expire_at": lease_token})

#
# {'token': 'J82DXAoDuzsiMAAw3V+WI+g/JfwtZJxCGE1nzDoELQ....', 'expire_at': 10799}
#
