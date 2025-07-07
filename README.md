# 🚀 Axum Socket.IO Chat Server

A modern, high-performance real-time chat server built with **Rust Axum** and **Socket.IO**, featuring room-based messaging, user management, and a beautiful web interface.

## ✨ Features

- **Real-time messaging** with Socket.IO WebSockets
- **Room-based chat** - Users can join/leave different chat rooms
- **User management** - Track online users and their status
- **Modern web interface** - Beautiful, responsive UI with Thai language support
- **RESTful API** - HTTP endpoints for user management and health checks
- **High performance** - Built with Rust and Axum for excellent performance
- **Cross-platform** - Works on Windows, macOS, and Linux
- **Docker support** - Easy deployment with Docker

## 🛠️ Tech Stack

- **[Axum](https://github.com/tokio-rs/axum)** - Modern async web framework for Rust
- **[socketioxide](https://github.com/Totodore/socketioxide)** - Socket.IO implementation for Rust
- **[Tokio](https://tokio.rs/)** - Asynchronous runtime for Rust
- **[Serde](https://serde.rs/)** - Serialization framework
- **HTML/CSS/JavaScript** - Frontend with Socket.IO client

## 🏗️ Architecture

```
┌─────────────────┐    WebSocket    ┌─────────────────┐
│   Web Browser   │ ◄──────────────► │   Axum Server   │
│  (Socket.IO)    │                 │  (socketioxide) │
└─────────────────┘                 └─────────────────┘
                                              │
                                              ▼
                                    ┌─────────────────┐
                                    │   In-Memory     │
                                    │   User Store    │
                                    └─────────────────┘
```

## 📦 Installation

### Prerequisites

- **Rust** (1.75 or later) - [Install Rust](https://rustup.rs/)
- **Cargo** (comes with Rust)

### Clone and Build

```bash
git clone <your-repo-url>
cd axum-socketio
cargo build --release
```

### Development Mode

```bash
cargo run
```

The server will start on `http://localhost:3000`

## 🚀 Usage

### Starting the Server

```bash
# Development mode
cargo run

# Production mode
cargo run --release

# Custom port
PORT=8080 cargo run
```

### Web Interface

1. Open `http://localhost:3000` in your browser
2. Enter your username and room name
3. Click "เข้าร่วมห้อง" (Join Room)
4. Start chatting!

### API Endpoints

#### Health Check

```http
GET /health
```

Response:

```json
{
  "status": "healthy",
  "service": "axum-socketio",
  "version": "0.1.0"
}
```

#### Get All Users

```http
GET /api/users
```

Response:

```json
[
  {
    "id": "socket_id_123",
    "username": "john_doe",
    "room": "general"
  }
]
```

#### Get Specific User

```http
GET /api/users/:user_id
```

Response:

```json
{
  "id": "socket_id_123",
  "username": "john_doe",
  "room": "general"
}
```

## 🔌 Socket.IO Events

### Client → Server Events

#### Join Room

```javascript
socket.emit("join_room", {
  username: "john_doe",
  room: "general",
});
```

#### Send Message

```javascript
socket.emit("send_message", {
  content: "Hello, World!",
  room: "general", // optional, uses current room if not specified
});
```

#### Leave Room

```javascript
socket.emit("leave_room");
```

### Server → Client Events

#### Welcome Message

```javascript
socket.on("welcome", (data) => {
  console.log(data.message); // "Welcome to Axum Socket.IO server!"
});
```

#### Room Joined

```javascript
socket.on("room_joined", (data) => {
  console.log(`Joined room: ${data.room}`);
});
```

#### New Message

```javascript
socket.on("new_message", (data) => {
  console.log(`${data.username}: ${data.content}`);
});
```

#### User Joined/Left

```javascript
socket.on("user_joined", (data) => {
  console.log(`${data.username} joined the room`);
});

socket.on("user_left", (data) => {
  console.log(`${data.username} left the room`);
});
```

## 📊 Performance

- **Concurrent connections**: Supports thousands of concurrent WebSocket connections
- **Memory usage**: Efficient in-memory user store
- **Latency**: Sub-millisecond message routing
- **Throughput**: High message throughput with Tokio async runtime

## 🔧 Configuration

### Environment Variables

- `PORT` - Server port (default: 3000)
- `RUST_LOG` - Log level (default: info)

### Custom Configuration

You can modify the server behavior by editing `src/main.rs`:

- Change CORS settings
- Modify static file serving
- Add authentication
- Implement database persistence

## 🐳 Docker Deployment

### Build Docker Image

```bash
docker build -t axum-socketio .
```

### Run Container

```bash
docker run -p 3000:3000 axum-socketio
```

### Docker Compose

```yaml
version: "3.8"
services:
  axum-socketio:
    build: .
    ports:
      - "3000:3000"
    environment:
      - RUST_LOG=info
```

## 🛡️ Security Features

- **CORS configured** for cross-origin requests
- **Input validation** on all Socket.IO events
- **Error handling** for malformed requests
- **Rate limiting** (can be added)
- **Authentication** (can be implemented)

## 📝 Development

### Project Structure

```
axum-socketio/
├── src/
│   └── main.rs          # Main application code
├── static/
│   └── index.html       # Web interface
├── Cargo.toml           # Dependencies
├── .gitignore          # Git ignore rules
├── README.md           # This file
└── Dockerfile          # Docker configuration
```

### Adding Features

1. **Database Integration**: Replace in-memory storage with PostgreSQL/MongoDB
2. **Authentication**: Add JWT or session-based auth
3. **Rate Limiting**: Implement message rate limiting
4. **File Upload**: Add file sharing capabilities
5. **Private Messages**: Direct messaging between users

### Testing

```bash
# Run tests
cargo test

# Run with coverage
cargo tarpaulin
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if needed
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Amazing web framework
- [socketioxide](https://github.com/Totodore/socketioxide) - Socket.IO for Rust
- [Tokio](https://tokio.rs/) - Async runtime
- [Socket.IO](https://socket.io/) - Real-time communication

---

## 📞 Support

If you have any questions or need help:

1. Check the [Issues](https://github.com/your-username/axum-socketio/issues) page
2. Create a new issue if your problem isn't already reported
3. Join our community discussions

**Happy coding! 🦀✨**
