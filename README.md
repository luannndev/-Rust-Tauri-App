
# RustDeskApp 🦀

This is a simple Rust-based desktop application built using the Tauri framework. Tauri allows you to create small, secure, and efficient desktop applications with Rust as the backend and modern web technologies for the frontend.

## Features ✨

- **Desktop Application**: Lightweight and secure desktop application with Tauri and Rust.
- **Rust Backend**: Efficient Rust backend handling core functionality.
- **Frontend (HTML/CSS/JavaScript)**: Modern UI using web technologies.
- **Logging**: Integrated logging system to track important events.
- **API Integration**: Example API calls from the Rust backend.
- **File Operations**: Demonstrates reading from and writing to files.

## Project Structure 📁

- **/src**: Contains the Rust backend source code.
  - `main.rs`: The main Rust application file.
- **/src-tauri**: Contains the Tauri-specific configuration and setup.
  - `tauri.conf.json`: Configuration for the Tauri application.
- **/public**: Contains the HTML/CSS/JavaScript files for the frontend.
  - `index.html`: The main UI file.
- **/logs**: Contains log files for the application.
- **Cargo.toml**: The Rust project configuration file.

## Prerequisites 🛠️

- **Rust**: Ensure that you have Rust installed on your system.
- **Tauri CLI**: You need to install the Tauri CLI to build and run the application. You can install it with:
  ```bash
  cargo install tauri-cli
  ```

## How to Run the Project 🚀

1. **Clone the repository**:
   ```bash
   git clone https://github.com/luannndev/-Rust-Tauri-App.git
   ```

2. **Install dependencies**:
   Navigate to the project folder and install the dependencies:
   ```bash
   cd rustdeskapp
   cargo tauri dev
   ```

3. **Build the application**:
   To build the application for production, run:
   ```bash
   cargo tauri build
   ```

## Features Breakdown 🔍

### 1. Desktop Application with Tauri
The project uses Tauri to create a lightweight desktop application. Tauri combines the power of Rust for system-level functionality with the flexibility of modern web technologies for the UI.

### 2. Rust Backend
The backend is written in Rust, leveraging the language's performance and memory safety. The backend handles tasks such as API calls, file reading, and logging.

### 3. Logging
The application writes logs to the `/logs` directory, allowing you to track important application events.

### 4. API Integration
An example API call is implemented in the Rust backend, demonstrating how the application can communicate with external services.

### 5. File Operations
The Rust backend can read from and write to files, showcasing how to manage file operations in Rust.

## License 📜

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing 🤝

Feel free to fork this repository, open issues, or submit pull requests. Any improvements or new features are welcome!
