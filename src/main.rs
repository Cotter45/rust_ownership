mod slice;

fn main() {
    // ownership

    // scope defines items on the stack for quicker access
    // string literal type, hardcoded str type - immutable
    let _s = "hello"; // global scope

    { 
        let _s = "hello"; // block scope, seperate variables and scope
    }

    // String type - mutable, stored on heap
    let mut mutable_s = String::from("hello");
    mutable_s.push_str(", world!"); // appends a literal to a String

    println!("{}", mutable_s); // prints "hello, world!"

    {
        let _s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid

    // if you want to keep a value on the heap in mem outside of scope by 
    // reassigning, you need to "clone" (deep copy) it. rust does not make 
    // shallow copies to avoid memory corruption on cleanup
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // this is different for scalar values placed on the stack
    // scalar :
    //  integers - ie u32
    //  booleans - ie true / false
    //  floats - ie f64
    //  characters - char
    //  tuples - only if they contain types above
    let x = 5;
    let y = x;
    
    println!("x = {}, y = {}", x, y);

    scope_test(); // can't use mem heaped variables after ownership is transferred

    transfer_return(); // values can also be passed back on function return

    pass_reference(); // to avoid passing and returning values so original can be used again in same scope

    slice::slice(); // function from slice.rs
}

fn scope_test() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{} - from takes_ownership", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{} - from makes_copy", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn transfer_return() {
    let _s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn pass_reference() {
    let s1 = String::from("hello");

    // passes to calculate length by reference so it can use its value
    // cannot modify something we are borrowing without making them mutable
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.


