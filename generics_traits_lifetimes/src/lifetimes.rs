
//lifetimes in Struct definitions:
struct ImportantExcerpt<'a> {
    part: &'a str,
}


// lifetimes in method definitions. In this case the lifetime must be specified on the impl since
// the lifetime parameter is part of the definition of the struct. 
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        return 3
    }
}

// ImportantExcerpt can now contain references as type parameters - the key idea here
// is that an instance of ImportantExcerpt cannot outlive the reference to its `part` parameter,
// since they have the same lifetime. 
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}


