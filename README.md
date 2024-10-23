# Leeca Core

**Leeca Core** is a powerful and extensible Rust framework designed to simplify the development of microservices and distributed systems. Built on top of **Axum**, it leverages asynchronous programming paradigms to deliver high-performance applications.

## 🚀 Features

- **Microservices Architecture**: Easily create and manage microservices with a focus on modularity and separation of concerns.
- **Event Sourcing**: Future-ready capabilities to support event-driven architectures, allowing you to track changes over time.
- **TCP Async Support**: Designed for high-concurrency applications, providing seamless communication over TCP.
- **Distributed Systems**: Built to scale and distribute workloads across multiple services and nodes.
- **Configurable Commands**: Define commands with structured configurations and descriptions for better usability and clarity.

## 🛠️ Future Plans

Leeca Core is continuously evolving. Upcoming features include:

- **Enhanced Support for Event Sourcing**: Streamlined mechanisms for implementing event sourcing patterns.
- **Advanced Communication Protocols**: Support for various protocols and messaging patterns, including pub/sub models.
- **Comprehensive Utilities**: Tools and libraries that cover everything needed for modern microservices and distributed system development.

## 🔗 Getting Started

To get started with Leeca Core, clone the repository and add it to your Rust project as a dependency.

```bash
git clone https://github.com/0x4rkh4m/leeca_core
```

Then add the following to your `Cargo.toml`:

```toml
[dependencies]
leeca_core = { path = "./leeca_core" }
```

## 🤝 Contributing

We welcome contributions! Please check our [Contributing Guidelines](CONTRIBUTING.md) for details on how to get involved.

## 📄 License

TBD

## 📜 Acknowledgements

Leeca Core is built on top of the following libraries and frameworks:

- [Axum](https://github.com/tokio-rs/axum): A powerful and flexible web framework for Rust.
- [Tokio](https://github.com/tokio-rs/tokio): A powerful and scalable runtime for asynchronous programming in Rust.
- [Serde](https://github.com/serde-rs/serde): A powerful and flexible serialization framework for Rust.
- [Config](https://github.com/mehcode/config-rs): A flexible and easy-to-use configuration library for Rust.