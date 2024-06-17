# Hackathon Starter App

This project is a hackathon starter app built with Rust using the Axum framework. It provides a ready-to-deploy solution with authentication, database integration, real-time communication, and several other features to kickstart your hackathon project.

## Features

- **JWT Authentication** and Google OAuth
- **PostgreSQL** for persistent storage
- **RESTful API** with CRUD operations
- **WebSockets** for real-time notifications
- **Bootstrap** for frontend
- **File Upload** support
- **Role-Based Access Control** (RBAC)
- **Rate Limiting** on API endpoints
- **Deployment** to Digital Ocean's App Center
- Integrations with **Stripe**, **Twilio**, and **OpenAI's ChatGPT**

## Project Structure

```plaintext
hackathon-starter-app/
├── src/
│   ├── auth.rs
│   ├── config.rs
│   ├── db.rs
│   ├── errors.rs
│   ├── main.rs
│   ├── middleware.rs
│   ├── models.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── projects.rs
│   │   ├── tasks.rs
│   └── ws.rs
├── templates/
│   ├── login.html
│   ├── register.html
│   └── index.html
├── static/
│   ├── css/
│   │   └── bootstrap.min.css
│   └── js/
│       └── bootstrap.min.js
├── migrations/
│   ├── 20220101010101_create_users_table.sql
│   ├── 20220101010201_create_projects_table.sql
│   ├── 20220101010301_create_tasks_table.sql
├── Cargo.toml
├── .env
├── Dockerfile (optional for local development)
├── README.md
└── .gitignore
