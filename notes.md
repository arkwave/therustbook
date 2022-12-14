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
- A reference is like a pointer in that it???s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

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
- important note about the the `?` operator - it can only be used in functions
  whose return type is compatible with the value the `?` is used on; `?`
  represents an early return from a function, and so the function has to
  actually offer that return type for its usage to be valid.
- the `?` operator can also be used with `Option` enums, example below. In this
  case, if the next() call returns a None, the function will return that; else,
  it will continue to read the rest of the characters in the line. 

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
- can define methods (i.e. `impl` blocks), `structs` and `enums` over **generic parameters**.
    - these generics are often included in the signature of the
      function/method/enum/struct so that the compiler "knows" that the type is
      being implemented as generic. 
    - there is no runtime penalty for generics in Rust - compiler performs
      **monomorphization**, i.e. given a function `func` generic over some type
      `T`, it checks to see: 
        1) what types the function is called on; say
      `float64` and `char`
        2) creates copies of `func` (e.g. `func_f64` and `func_char`) for each datatype. 
    As such, the cost is absorbed into the compile-time process and there is no runtime penalty. 
- the behavior can be further constrained by **traits**, i.e. types that implement a certain functionality.
- traits can have default implementations, which can then be over-ridden for each specific struct that trait is defined for.
    - refer to `main.rs` in `generics_traits_lifetimes` for an example; the
      `Summary` trait is instantiated with a default implementation, which is
      then augmented by `impl Summary for <struct_name>` to tailor the default
      to the requirements of each struct. 
- interestingly, traits can have multiple sub-methods; this makes them more
  similar to interfaces than properties of an object. 
- can additionally use **trait bounds** to constrain the types accepted by a
  function to include only those types that accept a certain trait. example:
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
    // snip 
}

```
- trait bounds also allow us to perform blanket implementations, i.e. implement
  traits for structs that already implement another trait. for example, the Rust
  standard library uses trait bounds to implement a `to_string` method for all
  types that implement the `Display` trait (code snippet below)
```
impl<T:Display> ToString for T {
    fn to_string(&self) {
        //--snip--
    }
}
```
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
- in this case, the `cmp_display` method is implemented only for `Pair` structs
  containing types `T` that implement the `Display` and `PartialOrd` traits.
- We can also define functions that return a type that implements a specific trait, for example something like:
```
fn returns_summarizable() -> impl Summary {
    Tweet { //tweet parameters }
}
```
- in this case, the function `returns_summarizable` returns a `Tweet` object,
  which we know implements the `Summary` trait. 
- **One caveat**: functions like this can only return a single `type`. There
  are workarounds which we will get to later. 

### 10.3: Lifetimes

#### Introduction & usage in function definitions
- the main purpose of **lifetimes** is to avoid dangling references, i.e. references that persist
after the variables they reference have gone out of scope.
- In some instances, lifetimes must be explicitly annotated - these instances are those where the compiler
cannot infer what the lifetime of a variable actually is. Example: consider a
case where we have two strings, `A` and `B`, and the following function: 
``` 
fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        return a
    } else {
        return b
    }
}
```
- trying to compile this code will throw an exception about lifetimes - tl;dr is that Rust has no idea
if the references it is supposed to "return" according to the function definition are valid anymore, because
it has no way to implicitly perform a lifetime assessment by looking at the scope of the variables.
- the solution here is to explicitly annotate the lifetimes as follows:
```
fn longest<'x>(a: &'x str, b:&'x str) -> &'x str {
    // --snip--
}
```
- here, the `'x` syntax tells the compiler "hey, each of these variables has a lifetime parametrized by `'x`"
- More examples of lifetime syntax:
    1. `&i32` - reference to an `i32`.
    2. `&'a i32` - reference to an `i32` with an explicit lifetime of `'a` 
    3. `&'a mut i32` - a mutable reference to an `i32` with an explicit lifetime of `'a`. 
- important thing to remember is that lifetimes describe the relationship between the lifetimes of multiple 
references to each other; for example,
``` 
fn longest<'a>(x: &'a str, y: str) -> &'a str {
    x
}
```
would compile just fine. The fact that no lifetime is specified on `y` indicates that the lifetimes
of `x` and `y` are not related at all, and the return value has a lifetime annotation of `'a` as well, so all is good.
However, the following

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("really_long_string");
    return result.as_str()
}
```
would fail to compile since the lifetime of `result` has no relationship whatsoever to 
the lifetime `'a` of `x` or `y`. Additionally, `result` goes out of scope at the end of 
the function, so the reference returned by `result.as_str()` is dangling. 

#### Usage in Enums and Structs
Key ideas here are as follows:

1. `impl` for structs containing lifetime annotations will also need lifetime annotations, since the compiler has to know
that the impl method is bound to the lifetime of the struct. Refer to `lifetimes.rs` for examples. 
2. **Lifetime Elision Rules**: these are a sequence of rules the compiler implements to decide if explicit lifetime annotation is required. 
3. the `static` lifetime - this lifetime is for variables that are valid for the entire lifetime of the program - kinda like constants in Python imported from some `constants.py` file. 


#### Tying it all together:
So how would we re-write a function returning a reference to the longest of two strings? The non-working implementation is reproduced below:

``` 
fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        return a
    } else {
        return b
    }
}
```

and here is a working example that brings together the concepts of trait bounds, lifetimes and generic type parameters:

```
use std::fmt::Display;

fn longest_with_an_annoucement<'a, T>(
    x: &'a str,
    y: &'a str:
    ann: T
) -> &'a str
where T: Display 
{
    println!("Attention, an announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }   
}
```
Since we're guaranteed that the generic T implements the Display trait by the
trait bound `T: Display`, we know that the call to the `println` macro will not
fail. Additionally, we know that all the references being handled by the
function have an explicit lifetime of `'a`, and so we can freely return the
references required provided the function is called with both references still
in scope. 


### 13.1: Closures
- closures are anonymous functions that 
1) can be assigned to variables and called regularly as functions
2) can capture the state of their their current execution environment; refer to the T-Shirt giveaway example. 

- example: `unwrap_or_else` as implemented for `Option` has the following definition:
```
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f:F) -> T 
    where F: FnOnce() -> T 
    {
        match self {
            Some(x) => x,
            None => f()
        }
    }
}
```
It accepts one argument: a closure that itself takes no arguments. It returns a
type T, which is the type stored inside the `Some` variant of an `Option`. If
`Option<T>` is `Some`, it returns `T`. Else, it returns `None`.  

- `FnOnce` is one of three **closure traits** that closures can implement, definitions below (lifted from The Book): 
1. `FnOnce` applies to closures that can be called at least once. All closures
   implement this trait, because all closures can be called. If a closure moves
   captured values out of its body, then that closure only implements FnOnce
   and not any of the other Fn traits, because it can only be called once.
2. `FnMut` applies to closures that don???t move captured values out of their
   body, but that might mutate the captured values. These closures can be
   called more than once.
3. `Fn` applies to closures that don???t move captured values out of their body
   and that don???t mutate captured values. These closures can be called more
   than once without mutating their environment, which is important in cases
   such as calling a closure multiple times concurrently. Closures that don???t
   capture anything from their environment implement `Fn`.

So in context: `unwrap_or_else` implements the `FnOnce()` closure trait with signature `()-> T`; this implies that the following must hold:
1. The closure can be called exactly once. 
2. The closure accepts no arguments. 
3. The closure must return the type `T` contained within the `Some` variant of the `Option` enum.

Refer to the Rust Book for an example where using the wrong kind of closure prevents code from compiling. TL;DR there has
to be a correspondence between what traits the closure implements and what the closure actually does; you can't use a closure
implementing `FnOnce` on multiple objects, and you can't use a closure implementing `FnMut` to move captured values out of 
the closure's environment.


### 13.2: Iterators 
- much like python - iterators in Rust are lazy, created with `.iter()` and implement a `next()` method that consumes them.
- **consuming adaptors**: methods that consume an iterator, i.e. the iterator cannot be used after the method is called. e.g. `sum()`.  
- **iterator adaptors**: methods that produce new iterators, i.e. applying a closure to the result of an iterator, e.g. `map()` 
- Key fact: in Rust, iterators do not produce anything unless consumed, so we can use the `collect()` method to evaluate the iterator.example:
``` 
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);
```

- `iter()` does not take control over the iterable it is called on, but `into_iter()` does. 

#### Using iterators and closures together
- can apply closures to iterators to elegantly and readably perform a sequence of operations. For example:
``` 
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

struct Shoe {
    size: u32,
    stype: String 
}

fn get_shoes_of_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

### 14: More about Cargo and Crates 


### 15: Smart Pointers
- **smart** pointers are pointers with additional metadata or guarantees.
    - example: `String` is a smart pointer, with the guarantee that its contents will be valid `UTF-8` strings. 
- Smart pointers are usually implemented as `structs` that implement the `Deref` and `Drop` traits.
    - `Deref` allows an instance of the smart pointer to work like a reference so that code can either run with a reference or with an instance of the struct.
    - `Drop` determines what happens when smart pointer instance goes out of scope.
- the `Box` smart pointer:
    - used to produce a finite-size stack-allocated pointer that points to data on the heap. 
    - benefit here is that heap allocated memory doesn't need to be moved around when passing ownership

#### Traits implemented by smart pointers:
1.`Deref`: 
    - this allows values "pointed to" by smart pointers to be referenced
      directly. Most in-built smart pointers (`Box`, `String`), etc implement
      this by default. 
    - Dereferencing is implemented by default on standard smart pointers and
      all "dumb" pointers; `Deref` only needs to be explicitly implemented if
      implementing anew storage class, or if we want to override the default
      behavior of `*` operator.  
    - Rust performs something called **deref coercion**, where it iteratively
      applies the deref operator to arguments until the value obtained matches
      the type desired by the function utilizing the value. For example,
      consider the following

    ```
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T; 

        fn deref(&self) -> &Self::Target {
            &self.0 
        } 
    }

    fn hello(x: &str) {
        println!("Hello {x}!"); 
    }

    fn main() {
        let m = MyBox::new(String::from("booyah"));
        hello(&m) // prints out "Hello booyah!"

    }
    ```
The above example only compiles because of deref coercion; we have the
following chain of deref calls: `&MyBox<String> -> &String -> &str`. The neat
thing is that there is **no runtime penalty** for deref coercion, since the
number of deref calls required is identified at compile time. 

2. `Drop`: 
    - this trait determines what happens when the variable implementing this trait goes out of scope.
    - is contained in the prelude, no need to bring it into scope. 
    - `a.drop()` cannot usually be called explicitly, because variables are
      cleaned up by default at the end of the scope - this would result in a
      double-freeing error, where Rust tries to free the same value twice. 
    - to drop variables explicitly before the end of scope, use the
      `std::mem::drop` instead; this is also in the prelude, can be called via
      `drop(a)` rather than `a.drop()`.

### 15.4: Reference Counting pointers
- reference counting pointers are specified by `Rc<T>`; this is a reference counting pointer that is generic over type T. 
- want to use these when we're not sure at compile time which part of our
  program will be the _last_ to use a specific heap-allocated piece of data.
  Example: assume we have a graph with a nodes and edges; we might want to
  clean up nodes that no longer have any edges associated with them. 
- Also implies that reference counting pointers are used to **share** data
  without being bound by compile-time restrictions re: ownership. Example:

``` 
enum List {
    Cons(i32, Box<List>), 
    Nil 
}

use crate::List::{Cons, Nil}

let A = Cons(5, Box::new(Cons(10, Nil)))
let B = Cons(20, Box::new(A));
let C = Cons(30, Box::new(A));
```
This example will throw a compile-time error - since the ConsList data structure owns its data by default, initializing B causes a move of A. Can resolve this with the following:

``` 
enum List {
    Cons(i32, Rc<List>), 
    Nil 
}

use crate::List::{Cons, Nil}
use std::rc::Rc;

let A = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
let B = Cons(20, Rc::clone(&A));
let C = Cons(30, Rc::clone(&A));
```
Every element of A is wrapped in an `Rc` so that the call to clone can effectively update the reference counters of each element.

TL;DR: Reference Counting Pointers allow you to use immutable references to share read-only data between different parts of your program. 

### 15.5 - 15.6: Interior Mutability & Memory leaks
- Interior mutability is a pattern in Rust that lets you mutate data even where
  there are immutable references to that data - this is usually disallowed, but
  we can get around the borrow checker by enforcing that memory checks are done
  at _runtime_ rather than _compile_time_.

- Rationale: compiler is conservative when it comes to ensuring that memory
  invariants are maintained. Example: you create a struct, and want to mock it
  out for testing purposes by fiddling with one of the internal values.

- Drawbacks: We don't get good error handling. Since the error is caught/found
  at runtime, it might break when deployed, and there's no other way to handle
  that case other than panic. 

- Refer to the book for more in-depth discussion - the example provided there is a little contrived, but does a decent job explaining the concept.

- Can use `RefCell` to implement the interior mutability pattern: 
1. `RefCell` has two safe methods; `.borrow()` and `borrow_mut()`, which return `Ref<T>` and `RefMut<T>` pointers respectively. Each of these implement `Deref`, so we can treat them as regular pointers. 
2. `RefCell` then tracks the number of `RefMut` and `Ref` pointers in existence - if memory invariants are violated, it panics the whole program 

- Crucially, we can use `RefCell` and `Rc` to share data across different parts of a program in a read-write manner, with the memory invariants enforced only at runtime.

- Using these together can result in _memory leaks_, i.e. situations where the
  pointer counts never go to 0 and so the memory can never be cleaned up.
  Simple example: imagine a ConsList that is self-referential in some way,
  implemented with `Rc` and `RefCell`. Specifically, `Rc` instances are cleaned
  up only when the number of *strong references* goes to 0.

- Solution: weak references, i.e. references that can refer to but cannot take ownership of values. 

### 16: Fearless Concurrency. 

