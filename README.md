![alt text](https://github.com/akropolisio/mailgun-sender/blob/master/img/web3%20foundation_grants_badge_black.png "Project supported by web3 foundation grants program")

# Mailgun Sender

This is Mailgun Sender.

# Status

POC. Active development.

# Building

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Build:

```bash
cargo build
```

# Run

```bash
cargo run
```

# Environment variables description
SERVER_IP - IP address for binding, e.g. 127.0.0.1

SERVER_PORT - port for binding, e.g. 8080

SECRET_PATH - path to directory with secrets, e.g. "secret"

# Secret files
$SECRET_PATH/login -  Mailgun API login, e.g. api, for details visite https://documentation.mailgun.com/en/latest/user_manual.html#sending-via-api

$SECRET_PATH/api_key - Mailgun API api_key, for details visite https://documentation.mailgun.com/en/latest/user_manual.html#sending-via-api

$SECRET_PATH/domain_name - Mailgun API domain_name, for details visite https://documentation.mailgun.com/en/latest/user_manual.html#sending-via-api
