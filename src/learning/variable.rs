pub fn start(){
    // Declare variable => Immutable by default
    let imut_var = 1;
    println!("This is Immutable variable: {}", imut_var);

    // Declare variable mutable
    let mut mut_var = 1;
    println!("This is Mutable variable: {}", mut_var);

    mut_var = 2;
    println!("This is Mutable variable: {}", mut_var);
    
    // Declare constant
    const PI: f64 = 3.14;
    println!("This is Constant variable: {}", PI);
}