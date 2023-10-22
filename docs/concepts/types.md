In Rust, data types are divided into several categories. Here's a comprehensive list, though it's important to note that Rust, being a systems programming language, allows for a wide range of custom and complex types beyond these. Below are the core data types:

## Scalar Types 
These represent a single value.

   ### Integer
They can be signed or unsigned, 8-bit to 128-bit, and arch-dependent types. 
    Examples:
        - i8
        - u8
        - i16
        - u16
        - i32
        - u32
        - i64
        - u64
        - i128
        - u128
        - isize
        - usize
   
   ### Floating-Point
There are two floating-point types, `f32` and `f64`, which are IEEE-754 single and double precision numbers, respectively.
   - f32
   - f64
   
   ### Boolean
The `bool` type, which represents `true` or `false`.
   
   ### Character
The `char` type, which represents a single Unicode scalar value.

---

## Compound Types
These group multiple values into one type.

   ### Tuples
A fixed-size ordered list of elements of potentially different types, e.g., `(i32, f64, u8)`.
   
   ### Arrays
A fixed-size ordered list of elements of the same type, e.g., `[i32; 5]` for an array of five `i32` values.

---

## Custom Types
These are user-defined types.

   ### Structs
Define a structure with named fields. Structs can be classic C structs, tuple structs, or unit structs.
   
   ### Enums
Allow you to define a type by enumerating its possible variants.

---

## Text Types
   ### String Slice
The `str` type, usually seen in its borrowed form `&str`, is a string slice representing a view of a string (immutable).
   
   ### String
The `String` type, which is a growable, mutable, owned, UTF-8 encoded string type.

___


## Pointers and References
   ### References
Borrowing allows you to refer to some value without taking ownership of it, and comes in two forms: `&T` (immutable reference) and `&mut T` (mutable reference).
   
   ### Raw Pointers
Similar to pointers in C, they come in two forms: `*const T` (immutable) and `*mut T` (mutable). They are not safe and their use is generally discouraged without good reason.

## Function Types
   - Functions also have types in Rust, including the return type of the function, and a function pointer might look like `fn(i32) -> i32` for a function that takes an `i32` parameter and returns an `i32`.

--- 

## Compound Types
   ### Slices
A dynamically-sized view into a contiguous sequence, written as `[T]`. It's more common to see `&[T]` for a reference to a slice.
   
   ### Tuples
A general way of grouping together some number of other values with a variety of types into one compound type.

---

## Special Types
   ### Never
The `!` type, also called the "never" type, represents a value that will never be returned.

---

## Smart Pointers (more complex types used for various forms of ownership):
   ### Box<T>
For allocating values on the heap.
   ### Rc<T>
A reference-counted type for shared ownership.
   ### Arc<T>
A thread-safe reference-counted type for shared ownership.
   ### Ref<T> and RefMut<T>
Types for borrowing shared or mutable references to the inside of a `RefCell<T>`.

--- 

## Concurrency Types
  ### Mutex<T>
Mutual exclusion primitive useful for protecting shared data.
  ### RwLock<T>
A read-write lock.

---

## Collections
  ### Vec<T>
A growable array type.
  ### HashMap<K, V>
A hash map (also known as a dictionary).
  ### BTreeMap<K, V>
A map based on a B-Tree.
  ### HashSet<T>
A set based on a hash table.
  ### BTreeSet<T>
A set based on a B-Tree.
  ### LinkedList<T>
A doubly-linked list.

These types form the building blocks for defining complex data structures and operations in Rust. The language's focus on safety and concurrency determines how these types can be used together, especially concerning mutability and ownership.