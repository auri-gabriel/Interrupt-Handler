# Interrupt Handler

## **⚠️ Disclaimer: This project is for educational purposes only.**

This is a program in Rust that uses threads and synchronization primitives to manage interrupts.

## Description

The `Interrupt` struct represents a single interrupt and contains an ID and a priority. The `InterruptHandler` struct contains a vector of `Interrupt`s and implements methods for receiving and handling interrupts. The `receive_interrupt` method takes an interrupt as input, adds it to the vector of interrupts, sorts the vector by priority, and prints out the updated list of interrupts. The `handle_interrupts` method manipulates the interrupts in the vector by handling the interrupt with the highest priority until the vector is empty.

## Usage

To run the program, you can use the following command in your terminal:

```
cargo run
```

## Contributing

If you would like to contribute to this project, please feel free to create a pull request.
