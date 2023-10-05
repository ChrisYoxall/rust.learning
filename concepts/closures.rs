// To run, from the command line do: rustc closures.rs && ./closures

fn main() {
    // Rust supports closures, which are anonymous functions that can be saved to a variable or
    // passed as arguments to other functions. Closures can capture values from the scope in
    // which they're defined.

    // Define a closure by using the pipe character | to denote the parameters, followed by the
    // body of the closure. A multi-statement body can be wrapped in brackets. Note that the return
    // type is inferred in this example and when it needs to be specified brackets must be used.
    // The type of closure parameters can also be inferred which differs to functions where the
    // type of the parameter must be specified.
    let is_even = |x: i32| x % 2 == 0;
    println!("Is 5 even? {}", is_even(5));

    // Define a closure that adds two numbers.
    let add = |n1, n2| n1 + n2;
    let mut sum = 0;
    for n in 1..6 {
        sum = add(sum, n);
    }
    println!("The sum of the numbers from 1 to 5 is {}", sum);

    // The above examples don't capture the enclosing environment, but does show how closures can
    // be used as an alternative to functions.
    // This example shows outer_var being captured by the closure.
    let outer_var = 5;
    let closure = |x| x + outer_var;
    println!("The result of the closure is {}", closure(5));

    // Trying to declare an inner function to do the same as the closure above will fail as inner
    // functions can't capture variables from the enclosing environment.
    //fn add(x: i32) -> i32 { x + outer_var };

    // To force the closure to take ownership of values it uses in the environment, even though the
    // body of the closure does not strictly need ownership, you can use the move keyword.
    let data = vec![1, 2, 3];
    let closure = move || println!("captured {data:?} by value");
    closure();
    // println!("data is {:?}", data); // Will fail at compile as data was moved into the closure

}
