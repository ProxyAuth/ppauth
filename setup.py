from setuptools import setup, find_packages

setup(
    name="ppauth",
    version="0.1.7",
    packages=find_packages(),
    install_requires=[
        "requests",
        "qrcode[tty]",
    ],
    entry_points={
        'console_scripts': [
            'ppauth = ppauth.cli:main',
        ],
    },
)
