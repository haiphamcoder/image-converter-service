# Project: Clean Architecture Rust Service

## Overview

This project demonstrates a **Clean Architecture** approach for building a backend service in Rust. It includes features such as encoding and decoding images, modular design, and RESTful API support. The project leverages `Actix-web` for the web framework and uses `Docker` for containerization.

## Project Structure

The project is organized into the following directories:

```text
.
├── Cargo.lock
├── Cargo.toml
├── docker-compose.yml
├── Dockerfile
├── README.md
└── src
    ├── adapters
    │   ├── api
    │   │   ├── image
    │   │   │   ├── image_controllers.rs
    │   │   │   └── mod.rs
    │   │   ├── mod.rs
    │   │   ├── ping
    │   │   │   ├── mod.rs
    │   │   │   └── ping_controller.rs
    │   │   └── shared
    │   │       ├── mod.rs
    │   │       ├── response.rs
    │   │       └── routes.rs
    │   ├── mod.rs
    │   └── spi
    │       ├── file_system
    │       │   ├── file_repository.rs
    │       │   └── mod.rs
    │       └── mod.rs
    ├── application
    │   ├── mod.rs
    │   └── usecases
    │       ├── decode_image_usecase.rs
    │       ├── encode_image_usecase.rs
    │       └── mod.rs
    ├── domain
    │   ├── entities.rs
    │   └── mod.rs
    ├── infrastructure
    │   └── mod.rs
    ├── main.rs
    └── mod.rs
```

### Key Components

- **adapters**: Contains APIs and external service integrations.
- **application**: Contains use cases and business logic.
- **domain**: Defines core entities and domain logic.
- **infrastructure**: Contains configuration and utility services.

## Prerequisites

- Rust programming language: [Install Rust](https://www.rust-lang.org/tools/install)
- Docker: [Install Docker](https://docs.docker.com/get-docker/)
- Postman (optional): For testing APIs.

## Setup Instructions

### 1. Clone the repository

```bash
git clone git@github.com:haiphamcoder/image-converter-service.git image_converter_service
cd image_converter_service
```

### 2. Configure the Environment

Create a `.env` file in the root directory with the following content:

```env
SERVICE_PORT=8081
```

### 3. Build and Run the Service

#### Using Docker

1. Build and run the container:

   ```bash
   docker-compose up --build
   ```

2. The service will be accessible at `http://localhost:8081`.

#### Without Docker

1. Install dependencies:

   ```bash
   cargo build --release
   ```

2. Run the application:

   ```bash
   cargo run --release
   ```

## API Endpoints

### 1. Ping Endpoint

- **GET** `/api/v1/ping`
  - Returns a health check response.

### 2. Encode Image

- **POST** `/api/v1/image/encode`
  - Request Body (JSON):

    ```json
    {
        "file_path": "path/to/your/image.jpg"
    }
    ```

  - Response:

    ```json
    {
        "base64_data": "<base64-encoded-string>"
    }
    ```

### 3. Decode Image

- **POST** `/api/v1/image/decode`
  - Request Body (JSON):

    ```json
    {
        "base64_data": "<base64-encoded-string>",
        "output_path": "path/to/output/image.jpg"
    }
    ```

  - Response:

    ```text
    Image saved successfully
    ```

## Testing

### Using Postman

1. Import the API endpoints into Postman.
2. Set the `Authorization` header to `Bearer valid_token`.
3. Test the endpoints as described above.

### Using Integration Tests

Run the tests using:

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

## Contributing

Feel free to open issues or submit pull requests for improvements or bug fixes. Feedback is welcome!
