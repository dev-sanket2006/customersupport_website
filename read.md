# SaaS Customer Support Ticketing System (Rust + Axum)

## 🌐 Tech Stack
- Rust (2024 Edition)
- Axum (v0.7)
- SQLx with PostgreSQL
- JSON Web Tokens (JWT)
- Postman Tested ✅

## ✅ Features
- User Authentication (JWT)
- Role-based Agent Support
- Create / Update / Delete Support Tickets
- Notes & Internal Communication
- Multi-Agent Collaboration
- Knowledge Base with Tags
- Reports Dashboard (Overview, by Agent, by Status)
- Full RESTful API

## 🔧 How to Run
1. Add your `.env`:
DATABASE_URL=postgres://postgres:<yourpass>@localhost:5432/<yourdb>
JWT_SECRET=supersecurekey

markdown
Copy code

2. Build & Run
cargo build
cargo run

3. Open Postman and hit:  
`http://127.0.0.1:3000/`

## 📬 Author
Sanket Suman Bastia (B.Tech CSE, SiliconTech)
