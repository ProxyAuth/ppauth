import ppauth
import requests

ppauth.auth(
    host="127.0.0.1", port=8080,
    username="admin", password="admin123",
    timezone="Europe/Paris"
)

token = ppauth.token()
# or use renew method
token = ppauth.token(renew=True) # Automatically re-authenticates and returns a new token if the previous one has expired.
lease_token = ppauth.lease_token()

print({"token": token, "expire_at": lease_token})

#
# {'token': 'J82DXAoDuzsiMAAw3V+WI+g/JfwtZJxCGE1nzDoELQ....', 'expire_at': 10799}
#
# Send your request
requests.get("https://127.0.0.1:8080/app", headers={"Authorization": "Bearer {}".format(token)}, verify=False)



