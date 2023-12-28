// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    /* 
        &'static str is a reference to a string slice with a 'static lifetime; guaranteed 
        to be valid for the whole duration of the program.
    */
    let mut shopping_list: Vec<&'static str> = Vec::new(); 
    // 'static is a lifetime specifier, often used for literals
    shopping_list.push("milk"); // "milk" is a string slice
}
