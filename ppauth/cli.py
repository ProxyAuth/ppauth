#!/usr/bin/env python3

import argparse
import requests
import qrcode
import sys
from qrcode.constants import ERROR_CORRECT_L
import urllib3

urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)

def generate_qrcode(data: str):
    qr = qrcode.QRCode(
        version=1,
        error_correction=ERROR_CORRECT_L,
        box_size=10,
        border=2
    )
    qr.add_data(data)
    qr.make(fit=True)

    print("\nScan the QR code above to register your TOTP secret.\n")
    qr.print_ascii(invert=True)

def generate_totp_qrcode(host: str, port: int, username: str, password: str, verify_tls: bool):
    url = f"https://{host}:{port}/adm/auth/totp/get"
    try:
        response = requests.post(url, json={
            "username": username,
            "password": password
        }, verify=verify_tls)

        if not response.ok:
            print(f"Error: Server returned {response.status_code}")
            sys.exit(1)

        data = response.json()
        uri = data.get("otpauth_uri")

        if not uri:
            print("Error: 'otpauth_uri' not found in response")
            sys.exit(1)

        generate_qrcode(uri)
    except Exception as e:
        print(f"Request failed: {e}")
        sys.exit(1)

def main():
    parser = argparse.ArgumentParser(description="ProxyAuth CLI")
    subparsers = parser.add_subparsers(dest="command")

    registry_parser = subparsers.add_parser("registry", help="Registry actions")
    otp_parser = registry_parser.add_subparsers(dest="action")

    otp_cmd = otp_parser.add_parser("otp", help="Generate TOTP QR code")
    otp_cmd.add_argument("--username", required=True, help="Username")
    otp_cmd.add_argument("--password", required=True, help="Password")
    otp_cmd.add_argument("--host", required=True, help="Hostname (e.g. 127.0.0.1)")
    otp_cmd.add_argument("--port", type=int, required=True, help="Port (e.g. 8080)")
    otp_cmd.add_argument("--no-tls-verify", action="store_true", help="Disable TLS certificate verification")

    args = parser.parse_args()

    if args.command == "registry" and args.action == "otp":
        verify_tls = not args.no_tls_verify
        generate_totp_qrcode(args.host, args.port, args.username, args.password, verify_tls)
    else:
        parser.print_help()

if __name__ == "__main__":
    main()
