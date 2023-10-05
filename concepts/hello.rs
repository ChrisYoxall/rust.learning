// To run, from the command line do: rustc hello.rs && ./hello

fn main() {
    // The exclamation mark denotes that println is a macro
    println!("Hello, World!");

    // Don't need to specify the type (i32) as it will be inferred by the compiler.
    let a: i32 = 10;
    let b: i32 = 20;
    let sum: i32 = add(a, b);
    println!("The sum of {} and {} is {}", a, b, sum);

    //a = a + 1; // This would throw an error as a is immutable

    // Rust is immutable by default. It's key to its reliability and safety goals. It forces
    // developers to think about how state should be mutated. In above example to make a mutable
    // declare it like: let mut a = 10;

    // In general the Rust compiler won't make any assumptions about the code, and will throw
    // errors if there is any ambiguity.

    // Rust has two compound types, those being tuples and arrays.

    // A tuple is a general way of grouping together a number of values with a variety of types into
    // one compound type. Tuples have a fixed length, once declared they cannot grow or shrink, and
    // in the example below the types can be inferred.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The first value in the tuple is {}", tup.0);

    // Use pattern matching to destructure a tuple. The underscore denotes we wont be using
    // those variables.
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    // Declare an array
    let values_array = [1, 2, 3, 4, 5];
    println!("The first value in the array is {}", values_array[0]);

    let mut sum_array = 0;
    for n in values_array {
        sum_array = add(sum_array, n);
    }
    println!("The sum of the values in the array is {}", sum_array);

    // Rusts range expressions are inclusive on the lower bound and exclusive on the upper bound
    // so the below will print 1, 2
    for i in 1..3 {
        println!("{}", i);
    }

    // Rust has array slices which are references to a contiguous sequence of elements in an array.
    let mut sum_slice = 0;

    for n in &values_array[0..2] {
        sum_slice = add(sum_slice, *n); // Use * to dereference the pointer
    }

    for n in &values_array[2..5] {
        sum_slice = add(sum_slice, *n);
    }

    println!("The sum of the two array slices is {}", sum_slice);

    // Rusts arrays can not grow once declared. Rust does include a number of collection types
    // such as a vector (vec) which is a growable array type.
    let mut values_vector = vec![1, 2, 3, 4];
    let mut sum_vector = 0;
    values_vector.push(5);
    for n in values_vector {
        sum_vector = add(sum_vector, n);
    }
    println!("The sum of the values in the vector is {}", sum_vector);

    // Refer to https://doc.rust-lang.org/std/collections/index.html for more information on
    // collections.

}

fn add(n1: i32, n2: i32) -> i32 {
    // Can place a 'return' statement here, but it's not necessary. The return value of a function
    // is the value of the final expression evaluated in the function body.
    n1 + n2
}
