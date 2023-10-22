# Rust Playground

I write in PHP, Ruby, Python, Javascript and Typescript. I wanted to learn Rust and here are some of the ways that Rust is different from the languages I know. 

### Unique or More Pronounced in Rust:

1. **Ownership and Borrowing**: 
   - These are cornerstone concepts in Rust, ensuring memory safety without a garbage collector. Understanding how ownership, borrowing, and lifetimes work will be crucial. These concepts are unique to Rust and are fundamental to its approach to memory management.

2. **Lifetimes**:
   - This is related to the borrowing system. Lifetimes are Rust's way of ensuring that references to memory are valid and help prevent dangling references. You'll need to understand how to annotate lifetimes and how the compiler uses them to enforce memory safety.

3. **Immutability by Default**:
   - In Rust, all variables are immutable by default, which is a stronger stance compared to the languages you listed. You must explicitly declare variables as mutable with `mut`.

4. **Pattern Matching**:
   - While found in some other languages, pattern matching is used extensively in Rust. It goes beyond simple value matching and can destructure, match, and bind values in more complex ways.

5. **Algebraic Data Types (Enums and Structs)**:
   - Rust’s enums are more powerful than typical C-style enums, allowing for 'enums with data' similar to algebraic data types in functional languages. Combined with pattern matching, they enable very expressive and type-safe code.

6. **Error Handling**:
   - Rust doesn’t have exceptions. Instead, it has the `Result` type for recoverable errors and the `panic!` macro for unrecoverable errors. Learning the idiomatic way of handling errors in Rust (using `match`, `Result`, and `Option`) will be new.

7. **Zero-Cost Abstractions**:
   - Rust emphasizes zero-cost abstractions, meaning that higher-level abstractions you use in your code compile down to minimal possible low-level code, ensuring that you don’t pay a performance penalty for abstractions.

8. **Macros**:
   - While you might be familiar with macros from other languages, Rust macros are more akin to meta-programming, allowing for code that writes other code (hygienic macros), which can be a powerful tool for developers.

9. **Concurrency without Data Races**:
   - Rust's approach to concurrency is unique because its type system and ownership model are designed to prevent data races at compile time.

### Common or Somewhat Familiar Concepts:

1. **Functions and Modularization**:
   - Functions, modules, and public/private encapsulation are common in programming, and Rust has its nuances, but the general principles are the same.

2. **Iterators and Closures**:
   - These are present in JavaScript, Python, and Ruby in various forms. Rust’s iterators and closures might feel familiar, but they are more functionally inclined, similar to those found in functional programming languages.

3. **Traits (Similar to Interfaces or Mixins)**:
   - Traits in Rust are somewhat analogous to interfaces in TypeScript or other OOP languages. They are used to define shared behavior and can be used as a form of polymorphism.

4. **Basic Control Flows**:
   - Loops, conditionals, and function calls work with the same basic logic across all these languages, even if the exact syntax and best practices may differ.

5. **Package Management (Cargo)**:
   - If you've used npm for JavaScript/TypeScript or gems for Ruby, Cargo serves a similar purpose for Rust, managing project dependencies and building your project.

6. **Testing**:
   - Automated testing concepts are quite universal. Rust provides good support for writing automated tests, similar to what you might find in other modern programming languages.

### Suggested Questions to Ask While Learning Rust:

1. How does ownership in Rust eliminate the need for a garbage collector?
2. How do lifetimes contribute to memory safety, and how can I infer the lifetimes without explicit annotations?
3. How do I handle errors in Rust in the absence of exceptions?
4. How can I use pattern matching with enums and structs for more expressive code?
5. How do I write concurrent Rust code and leverage its safety guarantees?
6. How do I use traits for code reuse and polymorphism?
7. How do I write and understand macros in Rust, and what are their advantages over functions?
8. How does immutability contribute to Rust's safety and concurrency features?
9. How do I manage dependencies and build my project using Cargo?
10. What are the best practices for using mutable state, references, and borrowing?
