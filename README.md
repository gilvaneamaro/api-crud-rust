## CRUD Rest API 
This project is a backend application developed in Rust, using a PostgreSQL database distributed across two Docker containers. The application supports CRUD operations and has been tested using Postman.

## Table of Contents

- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Running the Application](#running-the-application)
- [Running Tests](#running-tests)
- [Endpoints](#endpoints)
- [Credits](#credits)
- [License](#license)

## Getting Started

These instructions will guide you on how to set up and run the project on your local machine for development and testing purposes.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/get-started)
- [Postman](https://www.postman.com/downloads/)

## Installation

1. Clone the repository:
   ```sh
   git clone git@github.com:gilvaneamaro/api-crud-rust.git
   cd api-crud-rust
   ```

2. Build the Docker containers:
   ```sh
   docker-compose up --build
   ```

3. Set up the database:
   ```sh
   docker-compose exec db psql -U postgres -c "CREATE DATABASE db;"
   ```

## Running the Application

To start the application, run the following command:

```sh
docker compose build
```

The application will be available at `http://localhost:8000`.

## Endpoints

The application supports the following CRUD operations:

- **Create**: `POST /users/`
- **Read All**: `GET /users`
- **Read**: `GET /users/{id}`
- **Update**: `PUT /user/{id}`
- **Delete**: `DELETE /user/{id}`

### Example Request

#### Create an Item

```sh
curl -X POST http://localhost:8080/users -H "Content-Type: application/json" -d '{"name": "Gilvane", "email": "gilvane@email.com"}'
```

## Credits
This project was based on the guide created by Francesco Ciulla, you can find the video [here](https://youtu.be/vhNoiBOuW94).
.


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
