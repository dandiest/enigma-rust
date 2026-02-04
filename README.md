# 🦀 Rust Vault System

A CLI-based interactive security system built with Rust to master control flow and error handling.


## 🛡️ Apprentice Disclaimer 
I am young developer currently on a journey to master **Rust**. This project is part of my daily learning routine. 
It might not be "production-ready," but it represents my commitment to learning memory safety, low-level logic, and professional coding standards. 
**Feedback is always welcome!**


## 🚀 Features
- **Robust Error Handling:** The program handles invalid inputs (like strings instead of numbers) without crashing, using the `match` pattern.
- **Life Counter:** Implements a retry system where the user has 3 attempts before a lockout.
- **Infinite Loop Logic:** Uses the `loop` keyword to keep the interface active until a specific condition is met.

## 🛠️ Technical Implementation
* **Memory Safety:** Managed mutable string references for user input.
* **Pattern Matching:** Utilized `match` on `Result` types for clean and safe type conversion (`String` to `i32`).
* **Standard I/O:** Leveraged `std::io` for real-time terminal interaction.

## 📦 Getting Started
To run this project locally, ensure you have Rust installed and run:
```
cargo run
``` 

## ⚖️ License
This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
