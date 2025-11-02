# 🦀 Control Flow — Rust Programming Concepts

---

## 📘 Overview
This project demonstrates **control flow mechanisms** in Rust, including `if` expressions, `loop`, `while`, and `for` loops.  
It’s part of a Rust learning series exploring fundamental programming constructs in a clean, modular way.

---

## 🧩 Features
- 🧠 Conditional statements using `if`, `else if`, and `else`
- 🔁 Infinite and labeled loops
- 🔄 `while` and `for` iteration examples
- 🧮 Demonstrates `loop` with break values
- 🚀 Countdown logic using `.rev()` for reverse iteration

---

## 🗂️ Project Structure
Control_Flow/
├── src/
│ └── main.rs
├── target/
├── .gitignore
├── Cargo.toml
├── Cargo.lock
└── README.md

---

## ⚙️ Prerequisites
Make sure Rust and Cargo are installed. You can install them using **rustup**:

```bash
curl https://sh.rustup.rs -sSf | sh
```
Verify the installation:
```bash
rustc --version
cargo --version
```

---

## 🚀 Setup and Run
Clone the repository:

```bash
Copy code
git clone https://github.com/skipajenkins/Control_Flow.git
```
Navigate to the project folder:

```bash
cd Control_Flow
```
Build the project:

```bash
cargo build
Run the program:
cargo run
```

---

## 🧠 Code Overview
### 🔹 Conditional Statements
rust
```bash
if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```
You can also nest conditions:

rust
```bash
if another_number % 4 == 0 {
    println!("Number is divisible by 4");
} else if another_number % 3 == 0 {
    println!("Number is divisible by 3");
} else {
    println!("Number is not divisible by 4, 3, or 2");
}
```
###🔹 Loops
Basic loop:

rust
```bash
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
println!("The result is {result}");
```
Labeled loop:

rust
```bash
'counting_up: loop {
    println!("count = {count}");
    // Nested loop logic
}
```
### 🔹 While Loop
rust
```bash
let mut the_number = 3;

while the_number != 0 {
    println!("{the_number}");
    the_number -= 1;
}
println!("LIFTOFF!!!!");
```
### 🔹 For Loop
rust
```bash
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is: {element}");
}
```
Reversed range example:

rust
```bash
for number in (1..4).rev() {
    println!("{number}!");
}
println!("LIFTOFF!!!");
```

---

## 🧩 Concepts Covered
Concept	Description
### 🧮 If / Else	Conditional branching
### 🔁 Loop	Infinite and labeled loops
### ⏳ While	Repeated execution until condition fails
### 🔂 For	Iteration through arrays and ranges
### 💥 Break with Value	Exit loops and return data

---

## 🧰 Tech Stack
Language: Rust 🦀

Toolchain: Cargo

Version: Rust 1.81+

---

## 🪪 License
This project is licensed under the MIT License.

💡 Author
Created by @skipajenkins
Part of the Rust Learning Series exploring the language fundamentals.
