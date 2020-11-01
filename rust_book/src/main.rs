const EXAMPLE_CONST: &str = "Example Constant";

fn main() {
    println!("Hello, world!");
    ownership();
    control_flow(6);
    let result = statements_and_expressions();
    println!("result of statements_and_expressions: {}",result);
    var_and_mut();
}

fn ownership() {
    
}

fn control_flow(x: i32) {
    if x == 1 {
        println!("Looping one time")
    } else if x == 2 {
        println!("Looping two times")
    } else {
        println!("Looping three or more times")
    }

    let mut i = 0;
    while i < x {
        println!("Loop {}", i);
        i = i + 1;
    }
}

fn statements_and_expressions() -> i32 {
    // This is a statement because it does not return a value
    let y = {
        let x = 3;
        // This returns a value BECAUSE IT DOES NOT HAVE A SEMICOLON AT THE END
        x + 1
    };
    println!("The value of y is: {}", y);
    y+1
}

fn var_and_mut() {
    // This variable is immutable
    let x = 5;
    println!("The value of x is: {}", x);
    //So, this line will not compile
    // x = 6;
    // let mut makes a variable mutable
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    println!("The value of EXAMPLE_CONST is: {}", EXAMPLE_CONST)
}