# Random notes when reading through The Rust Programming Language.

### 3.2: Data Types 
- arrays are fixed size and stack-allocated; vectors are variable size and heap-allocated. 
- arrays must have elements of same type. 
- access elements in an array with x[0], x[1] etc (0-indexed)
    > in contrast to tuples, which are accessed as x.0, x.1 etc. 
- array types specified by: let a: [<type or value>]; <# elements>]
    > e.g. let a = [3; 5] outputs [3, 3, 3, 3, 3]
    > e.g. let a = [i32; 5] = [1, 2, 3, 4, 5]

### 3.3: Functions 
- distinction between _statements_ and _expressions_: 
    > expressions return values, statements do not. 
    > adding a semicolon to an expression turns it into a statement and so removes the return value. 
    > function calls, macro calls (like println! or vec!) and math operations (5 + 6) are all expressions. 

### 3.5: Control flow
- can use if, else if, and else statements. 
- non bool types (empty lists, 0, etc) are NOT converted to booleans and so cannot be used nakedly in if/else if statements. 
- variable assignment in the form of `let x = if <condition> {v1} else {v2}` only work if v1 and v2 are of the same type. 
- loop labels - can initialize loops of the form:

        ```
        fn main() {
            let mut count = 0;
            'counting_loop: loop {
                // do stuff 
                if some_condition {
                    break 'counting_loop
                }
            }
        }
        ```
- only new loop type is `loop`, which is just an infinite loop. regular while and for loops apply. 
- ranges: can specify a range by 
    ```
    for number in (1...=4).rev() {
        // do stuff
    }
    ``` 
    the equal sign at the end indicates this range included 4. dropping it would return 3, 2, 1


### 4.1: Ownership
- distinction between heap allocation and stack allocation:
  - stack = LIFO style. very quick inserts and pops, caveat is that everything pushed to stack must be of fixed, known size.
  - heap memory is allocated by a memory allocator, which returns a _pointer_ (i.e. address) to the specific chunk of memory that has been allocated. Slower than accessing memory from stack, but allows for variable size.
  -  copying non-primitive variables is never done deeply. For example, 
  ``` 
  let s1 = String::from("hello"); 
  let s2 = s1 
  ```
  destroys the pointer to heap-allocated memory that s1 holds, and gives it over to s2. It is not merely a shallow copy, it is a _move_. Note the difference between "hello" and String::from("hello"); the former is stack-allocated
  and the latter is heap-allocated. To perform a deepcopy, can do something like 
  ``` 
    let s1 = String::from("hello");
    let s2 = s1.clone();
  ```

- passing a heap-allocated variable into a function passes _ownership_ of that variable to the function; it then goes out of scope altogether once the function finishes executing.
  - This is clearly not the case for stack-allocated variables, but the Type system will prevent a heap-allocated string (of type `String`) from being confused with an immutable reference to a stack allocated string (of type `&str`).
  
### 4.2: References and Borrowing
- A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

- We can have two kinds of references:
    - Many _immutable_ references;
    - One _mutable_ reference.

- These reference types are mutually exclusive; we cannot have mutable and immutable references to the same data co-exist; the immutable references don't expect the value of the underlying to change, while the mutable reference can do exactly that. 

- We also cannot have dangling references, i.e. references to variables that fall out of scope; all references must be **valid**.


### 4.3: The `slice` type. 
- slices are _references to collections of objects_; for example, given a string `let s = String::from("hello world")`, we can construct references to sub-parts of the string as follows:
    ```
    let hello = &s[0..5];
    let world = &s[6..11];

    ```
- all string literals are de-factor slices; this is because string literals are stack-allocated and so of a fixed size, i.e. a string literal is just a slice pointing to a set of addresses in memory.
- `&String -> &str` equivalence uses something called _implicit deref coercion_, god knows what the hell that means.


### 5.1 Defining and Instantiating Structs. 

- Structs are basically like FastAPI models - are named sets of key-value pairs that can be instantiated. They are used to group together items of data that make sense together.  
- Like other types, passing a struct into a functiong gives that function ownership over the struct - can give a reference if we do not want this. 
- Structs are either wholly mutable or wholly immutable; cannot make certain fields within a struct mutable. The mutability is derived from the variable assignment, e.g.:

    ```
    struct A {foo: i32, bar: i32}; 
    let a1 = A {foo: 5, bar: 6); // <- this struct is immutable, since a1 is immutable. 

    let mut a2 = A {foo: 7, bar: 10} // <- this struct is mutable, since a2 is mutable. 

    ```
- Perfectly legal to define structs with no fields. 
- **General Principle**: we want a struct to **OWN** all of its data as far as possible - possible to instantiate fields as references, but this requires specifiying _lifetimes_.  
- Useful syntax:
    - `dbg!()` macro - takes ownership of an expression, prints file and line number, and then returns ownership. 
    - can specify `#[derive(Debug)]` on a struct to use println! statements - this implements the Debug trait on the struct defined. 

- Structs can have two kinds of methods: 
    - instance methods. these take some variant of `self, &self, &mut self` as arguments. Called by `instance.method()`.  
    - classmethods, which do not. Called by `Struct::method`.


### 6.1 Enums & Pattern Matching 
- Can use enums to define multiple variants of the same underlying `Type` quickly. 
- Each variant can have its own constructors/data types as input. 
- Can instantiate a variant by going `enum::variant(variant_data_input)`
- Can define methods on enums as well, which will then be accessible by every variant of the enum. 
- **Special**: The Option Enum
    - Rust has _no null values_; values that could potentially be `Null` are cast to an `Option<T>` type, where the type `Option` is generic over the type of the contained value `T`.  
    - The `Option` enum in turn has two variants: `Some` (which handles non-null cases) and `None`, which handles the null case. 
    - All operations that use the `Option` type must explicitly handle both `Some` and `None` cases; `Some` variants have access to the generic type `T` contained within.
- Can use match statements to enumerate and handle all variants within an enum.
    - matches are _exhaustive_; every case needs to be handled. 
    - cases that don't need to be explicitly handled can be put under the "other" key, and cases to be ignored can be handled with `_ => handling_func()`
- if using a match is too verbose, can we instead use `if let` and `else` as syntactic sugar to achieve the same effect.


### 8: Common data structures/collections. 

- Strings: 
    - more complex in Rust than in other languages; cannot use integer slicing due to how Rust store strings in memory.
        > dependent on encoding; anything non-ASCII requires > 1 byte to encode for each char, so calling str[i] will not return the i-th character like we expect. 
        > generalizing across all byte requirements is tedious. 
        > solution: use iterators over characters in a string (e.g. `for c in "hello".chars()`) or bytes (e.g. for b in "hello".bytes()). 
    - to re-iterate: `&str` is a stack-allocated immutable reference to string literal of fixed size, `String` is a heap-allocated variable-length `String` object.

- Vectors:
    - heap-allocated, variable size; initialized with `vec![1, 2, 3]` etc. 
    - indexed with integers; for instance, given a vector `v`, can runs omething like `first_elem = &v[0]`. Note that we are indexing a reference to the vector. 
    - vectors also implement the `.get` method; i.e. `let first_elem = v.get(0)`; the get method handles the reference automatically. `.get` also returns None for entries
    outside the range of the vector automatically.
    - `.push` method appends to the vector. 
    - same rules regarding mutable and immutable references apply; cannot do something like:
        ```
        let first = &v[0]
        v.push(6);
        ```
    - In this case, `first` holds an immutable reference to the first element of the array; however, `push` requires a mutable reference to the array to actually
    insert the element - therefore the compiler will throw an error. 
    - can iterate over the values in a vector using the conventional `for i in (mut) &v` type syntax, depending on whether or not we want mutable or immutable references 
      to the contained elements. Note that if we're using a mutable reference to the contents of a vector, we need to _dereference_ the element when making changes to it. 
    - vectors can only store data of the same type; can extend flexibility by using enums (remember that all variants of an enum are the same type). 

### 9: Error Handling with Panic (unrecoverable)
- `panic!` macro can be used to raise the equivalent of RuntimeErrors; indicates that the error has put the program into an unrecoverable state of some sort. 
- can set the `RUST_BACKTRACE=1` environment variable to output the entire stack trace of a given error. 

### 9.2: Handling recoverable errors with `Result<T, E>`
- By default, many Rust functions return a `Result` object; this object is an enum with two variants, `Ok` and `Err`. The types of `Ok` and `Err` are inferred from the operation 
that is being handled; for example in the following snippet:

```
use std::fs::File; 
fn main() {
    let greeting_file_result = File::open("hello.txt")
}
```
the `File::open("hello.txt")` method returns a `Result<std::fs::File, std::io::Error>`. All variants of a `Result` must be explicitly handled, either by using `match` or by using closures (later chapter). Example of a closure is as follows:

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error); 
            })
        } else {
            panic!("Problem opening the file!: {:?}", error);
        }
    });
}
```
This snippet does the following: 1) tries to open the file 2) if an error is thrown, checks the error type 3) if the error type is FileNotFound, it creates the file 4) panic otherwise. 

#### `expect` and `unwrap`
- Instead of matching, we can use the `unwrap` and `expect` keywords to automatically panic if an error variant is observed. `unwrap` directly returns the `Ok` if found, otherwise
panics; `expect` displays a user-specified error message if an `Err` is found. In production, might want to use `expect` to display more information about what went wrong.  

``` 
let greeting_file = File::open("hello.txt").unwrap();
```
```
let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
```

#### Error Propagation
- errors can be propagated up using the `?` symbol - refer to `propagation.rs` for examples. 
- important note about the the `?` operator - it can only be used in functions whose return type is compatible with the value the `?` is used on; `?` represents an early
return from a function, and so the function has to actually offer that return type for its usage to be valid.
- the `?` operator can also be used with `Option` enums, example below. In this case, if the next() call returns a None, the function will return that; else, it will continue to
read the rest of the characters in the line. 

``` 
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last() 
}
```
- can define custom structs for field validation. For example, the struct below requires the guess be between 1 and 100: 
```
#![allow(unused)]
fn main() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
```

### 10.1 - 10.2: Generics & Traits
- can define methods (i.e. `impl` blocks), `structs` and `enums` over generic parameters. 
- the behavior can be further constrained by traits, i.e. types that implement a certain functionality.
- traits can have default implementations, which can then be over-ridden for each specific struct that trait is defined for. 
- interestingly, traits can have multiple sub-methods; this makes them more similar to interfaces than properties of an object. 
- can additionally use **trait bounds** to constrain the types accepted by a function to include only those types that accept a certain trait. example:
```
fn main<T: Summary> (item:&T) {
    //snip
}
```
can use the following syntatic sugar if we don't need to explicitly constrain the type:
```
fn main(item: &impl Summary) {
    //snip 
}
```
- To display multiple trait bounds, we can use the following `where` syntax:
```
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{

```
- trait bounds also allow us to perform blanket implementations, i.e. implement traits for structs that already implement a certain trait. for example,
rust standard library uses trait bounds to implement a `to_string` method for all types that implement the `Display` trait. 
- you can also use trait bounds to conditionally implement methods for types that implement certain traits. Example:

```
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
- in this case, the `cmp_display` method is implemented only for `Pair` structs containing types `T` that implement the `Display` and `PartialOrd` traits. 

### 10.3: Lifetimes 
