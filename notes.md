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




