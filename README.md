### Documentation to Markdown Converter

#### Table of Contents

    •	Introduction
    •	Features
    •	Architecture
    •	Prerequisites
    •	Usage
    •	Docker Deployment
    •	Configuration
    •	Logging
    •	Contributing
    •	License

#### Introduction

The AWS Documentation to Markdown Converter is a web application that allows users to convert AWS documentation pages into Markdown format. This tool simplifies the process of extracting information from AWS docs and integrating it into your own documentation or knowledge base.

#### Features

    •	Easy Conversion: Input an AWS documentation URL and receive the content in Markdown format.
    •	Web Interface: User-friendly frontend for entering URLs and viewing converted content.
    •	RESTful API: Backend API built with Rust and Actix-web for high performance and scalability.
    •	Dockerized Microservice: Easily deployable as a microservice using Docker and Docker Compose.
    •	Logging: Integrated logging for monitoring and debugging purposes.

#### Architecture

The application consists of two main components:

    1.	Backend:
    •	Language: Rust
    •	Framework: Actix-web
    •	Functionality:
    •	Receives POST requests with AWS documentation URLs.
    •	Fetches the HTML content of the provided URL.
    •	Converts the HTML content to Markdown using html2md.
    •	Returns the Markdown content as a JSON response.
    2.	Frontend:
    •	Technologies: HTML, CSS, JavaScript
    •	Functionality:
    •	Simple interface for users to input AWS documentation URLs.
    •	Sends the URL to the backend API.
    •	Displays the converted Markdown content to the user.

#### Prerequisites

    •	Rust: Version 1.75 or newer
    •	Docker: Version 20.10 or newer (for Docker deployment)
    •	Docker Compose: Version 1.29 or newer (optional, for managing multiple services)

#### Usage

##### Running Locally

1. Start the Backend Server

cargo run

The server will start and listen on http://127.0.0.1:8080.

2. Access the Frontend

Open your browser and navigate to http://127.0.0.1:8080.

3. Convert an AWS Documentation URL

   • Enter a valid AWS documentation URL into the input field.
   • Click the “Convert” button.
   • The converted Markdown content will be displayed on the page.

Running with Docker

1. Build the Docker Image

docker build -t aws_doc_to_md_backend .

2. Run the Docker Container

docker run -d -p 8080:8080 --name aws_doc_to_md_service aws_doc_to_md_backend

3. Access the Application

Open your browser and navigate to http://localhost:8080.

##### Docker Deployment

Using Docker Compose

1. Start the Services

docker-compose up -d

#### Contributing

Contributions are welcome! Please follow these steps:

    1.	Fork the Repository

Click the “Fork” button at the top right of the repository page. 2. Create a Feature Branch

git checkout -b feature/YourFeature

    3.	Commit Your Changes

git commit -am 'Add new feature'

    4.	Push to the Branch

git push origin feature/YourFeature

    5.	Open a Pull Request

Submit your pull request for review.

#### License

This project is licensed under the MIT License.

#### Acknowledgments

    •	Actix Web: A powerful, pragmatic, and extremely fast web framework for Rust.
    •	html2md: A Rust crate for converting HTML to Markdown.
    •	Reqwest: An ergonomic HTTP client library for Rust.

Happy Coding!
