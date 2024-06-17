# Rustathon

Rustathon is a hackathon starter app built with Rust and Axum. It includes JWT authentication, PostgreSQL integration, WebSockets, and more.

## Features

- **Authentication**: JWT and Google OAuth 2.0
- **Database**: PostgreSQL integration using Diesel
- **WebSockets**: Real-time updates with WebSockets
- **File Uploads**: Support for file uploads
- **Rate Limiting**: Rate limiting on API endpoints
- **APIs**: Stripe, Twilio, OpenAI's ChatGPT
- **Frontend**: Bootstrap for styling

## Getting Started

### Prerequisites

- Rust and Cargo installed
- PostgreSQL database
- Docker (optional, for containerization)

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/rustathon.git
   cd rustathon
   ```

2. Set up the environment variables
    ```sh
    cp .env.example .env
    # Edit the .env file to set your environment variables
    ```

3. Set up the database
    ```sh
    diesel setup
    diesel migration run
    ```

4. Run the application
    ```sh
    cargo run
    ```
### Building with Docker

1. Build the Docker image
    ```sh
    docker build -t rustathon .

2. Run the Docker container
    ```sh
    docker run -p 3000:3000 --env-file .env rustathon


