# Arbitrage System

![Rust CI/CD](https://github.com/mobistyle/arbitrage-system/workflows/Rust%20CI/CD/badge.svg)

## Important Security Notice
NEVER commit `.env` file or any files containing API keys to the repository.
Use `.env.example` as a template and create your own `.env` file locally.

## Setup
1. Copy `.env.example` to `.env`
2. Fill in your API keys in `.env`
3. Run `cargo build`
4. Run `cargo test`
5. Run `cargo run`

## Security Best Practices
- Keep your API keys secure and never share them
- Use API keys with minimal permissions needed
- Regularly rotate your API keys
- Monitor your API key usage
