# Common Rust Data Types

| Data Type | Rust Syntax | Example Value |
|---|---|---|
| Integer | `i32` | `10` |
| Float | `f64` | `3.14` |
| Boolean | `bool` | `true` |
| Character | `char` | `'A'` |
| String Slice | `&str` | `"Hello"` |
| String | `String` | `String::from("Hello")` |
| Tuple | `(i32, char)` | `(18, 'B')` |
| Array | `[i32; 3]` | `[1, 2, 3]` |

# Example

```rust
fn main() {
    let age: i32 = 20;
    let price: f64 = 9.99;
    let is_active: bool = true;
    let grade: char = 'A';

    let name: &str = "Alice";
    let city: String = String::from("Singapore");

    let person: (i32, char) = (18, 'B');

    let numbers: [i32; 3] = [1, 2, 3];
}
```

# Difference Between `&str` and `String`

| Type | Meaning | Can Change Size? | Stored Where |
|---|---|---|---|
| `&str` | String slice (reference to text) | No | Usually fixed text in program |
| `String` | Owned, growable string | Yes | Heap memory |

# `&str` Example

```rust
let name: &str = "Hello";
```

# What Does & Mean?
& means “reference”.

It borrows data instead of copying or owning it.

# What is : After Variable Name?
: means “type declaration”.

let variable_name: data_type = value;

```rust
let variable_name: data_type = value;
let age: i32 = 20;
```

# What is `mut` in Rust?

`mut` means "mutable".

It allows a variable value to change.

Without `mut`, variables are immutable by default.

## Without `mut`

```rust
let age = 20;

age = 21; // ERROR
```

# Ownership, References, and Borrowing in Rust

These are Rust memory safety rules.

---

# 1. Ownership

Each value in Rust has ONE owner.

When ownership moves, old variable cannot use it anymore.

In another words: In Rust, every piece of data belongs to one variable at a time.

## Example

```rust id="4rqzdi"
fn main() {
    let s1 = String::from("Hello");

    let s2 = s1;

    println!("{}", s2);

    // println!("{}", s1); // ERROR
}
```

# Rust Ownership Analogy

• Ownership  
You own the house key.  
The variable fully controls the value.

• Move  
You give the only key to someone else.  
Now they are the owner, and you cannot enter anymore.

• Borrow  
You let someone use the house temporarily, but you still keep ownership of the key.  
After they finish, control returns to you.

• Clone  
You create a completely new duplicate house with another key.  
Now both owners have independent houses.


# `()` in Rust

`()` is called the "unit type".

It means:

```text id="v26gbf"
No value
```

Similar idea to:

• void in C/C++

• null return concept in some languages

```rust
fn say_hi() {
    println!("Hi");
}

fn say_hi() -> () {
    println!("Hi");
}
```