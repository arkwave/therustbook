fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    
    let r1 = &s; 
    let r2 = &s;
    println!("{}, {}", r1, r2);  

    let r3 = &mut s;
    r3.push_str("...again?");

    //println!("{}", r1); <- this will break, since the reference is being used after mutable
    //borrow. 

    println!("{}", s);
    
    // println!("{}", r1); <- this will break, since r1 and r2 are out of scope after the mutable
    // borrow. 

}

fn change(string: &mut String) {
    string.push_str(", world!")
}
