# REV-ORP (Reverse Proxy with CAPTCHA Protection)

A lightweight, Rust-based reverse proxy server with built-in CAPTCHA verification to protect your web applications from automated access.

## Features

- **Reverse Proxy Functionality**: Forwards requests to your backend services
- **CAPTCHA Protection**: Simple math-based CAPTCHA challenge to verify human users
- **Session Management**: Uses cookies to remember verified users
- **Customizable Configuration**: Easy to configure backend URL and server settings

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/shri-acha/rev-orp.git
cd rev-orp
```

2. Build the project:
```bash
cargo build --release
```

### Running the Server

Run the server with default settings:
```bash
cargo run --release
```

By default, the server runs on `127.0.0.1:8080` and proxies to `127.0.0.1:4000`.

### Environment Variables

The server can be configured using environment variables:

- `HOST`: The host address to bind the server to (default: `127.0.0.1`)
- `PORT`: The port to listen on (default: `8080`)

Example:
```bash
HOST=0.0.0.0 PORT=3000 cargo run --release
```

## How It Works

1. When a user visits any protected route, they are redirected to a verification page
2. The user solves a simple math CAPTCHA (addition problem)
3. Upon successful verification, a cookie is set, and the user can access the protected content
4. The verification cookie expires after one hour

## Project Structure

- `src/main.rs` - Entry point and server configuration
- `src/proxy_server/mod.rs` - Proxy server setup and configuration
- `src/proxy_server/handlers/mod.rs` - Request handlers for proxy, verification page, and verification logic
- `src/proxy_server/static_page/verify_page.html` - CAPTCHA verification page template

## Configuration

The backend URL can be configured by modifying the `ProxyConfig` struct in `src/proxy_server/mod.rs`.

## Security Considerations

- The CAPTCHA challenge is a simple math problem, which provides basic protection
- The session cookie is set with HTTP-only flag for security
- The cookie will be set with secure flag when running over HTTPS

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
