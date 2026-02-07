# Todo API with Rocket (Rust)

A simple yet powerful Todo API built with the Rust programming language using the Rocket web framework. This project demonstrates building a RESTful API with CRUD operations and serving static files.

## Features
- RESTful API with full CRUD operations

- In-memory storage using Rust's Mutex for thread-safe operations

- JSON serialization/deserialization with Serde

- Static file serving for frontend hosting

- Simple and clean code structure

## API Endpoints
```
Method	Endpoint	Description
GET	/api/           status check
GET	/api/todos	Get all todos
POST	/api/todos	Create a new todo
PUT	/api/todos/<id>	Update a todo by ID
DELETE	/api/todos/<id>	Delete a todo by ID
```

## Todo Structure
```json
{
  "id": 1,
  "title": "Learn Rust",
  "completed": false
}
```

## Prerequisites
- Rust (latest stable version)

- Cargo (comes with Rust)

## Installation
1. Clone the repository:

```bash
git clone https://github.com/yourusername/rocket-todo-api.git
cd rocket-todo-api
```

Build the project:

```bash
cargo build
```
Run the server:
```bash
cargo run
```
The server will start at http://localhost:8000 by default.

```
Project Structure
text
├── Cargo.toml          # Rust dependencies and project configuration
├── src/
│   └── main.rs         # Main application code
├── static/             # Static files (HTML, CSS, JS)
└── README.md           # This file
```

## Usage Examples
Using cURL
Get all todos:
```bash
curl http://localhost:8000/api/todos
```
Create a new todo:
```bash
curl -X POST http://localhost:8000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"id":0,"title":"New Todo","completed":false}'
```
Update a todo:
```bash
curl -X PUT http://localhost:8000/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"id":1,"title":"Updated Todo","completed":true}'
```
Delete a todo:
```bash
curl -X DELETE http://localhost:8000/api/todos/1
```
## Using a Frontend

1. Create an index.html file in the static/ directory

2. Access it at http://localhost:8000/index.html

3. Use JavaScript fetch API to interact with the backend

## Dependencies
- rocket: Web framework for Rust

- serde: Serialization/deserialization framework

- rocket::serde: Rocket's integration with Serde

See Cargo.toml for complete dependency list and versions.

## Development
### Running in Development Mode
```bash
cargo run
```
Running in Release Mode
```bash
cargo run --release
```
Building for Production
```bash
cargo build --release
```
The executable will be in target/release/.

## API Design Notes
1. IDs are auto-incremented by the server on creation

2. PUT requests require the full object, not partial updates

3. 404 is returned for non-existent resources on PUT/DELETE

4. All data is stored in memory and will be lost on server restart

## Error Handling
- Missing resources return 404 Not Found

- Server errors return 500 Internal Server Error

- Invalid JSON returns 400 Bad Request

## Example Frontend Integration
Here's a simple JavaScript example to interact with the API:

```javascript
// Get all todos
fetch('/api/todos')
  .then(response => response.json())
  .then(todos => console.log(todos));

// Create a new todo
fetch('/api/todos', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    title: 'New Task',
    completed: false
  })
});
```

## Contributing
1. Fork the repository

2. Create a feature branch (git checkout -b feature/amazing-feature)

3. Commit your changes (git commit -m 'Add amazing feature')

4. Push to the branch (git push origin feature/amazing-feature)

5. Open a Pull Request

## Future Enhancements
Potential improvements:

- Database persistence (SQLite, PostgreSQL)

- User authentication

- Todo categories and tags

- Due dates and reminders

- Search and filtering

- Rate limiting

- OpenAPI/Swagger documentation

### License
This project is licensed under the MIT License - see the LICENSE file for details.

### Acknowledgments
- Rocket Web Framework

- Rust Programming Language

- Inspired by various Todo API tutorials and examples

### Support
For issues and questions, please open an issue in the GitHub repository.