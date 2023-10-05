// To run, from the command line do: rustc iterators.rs && ./iterators

fn main() {

    // The iterator pattern allows you to perform some task on a sequence of items in turn.
    // Rust provides iterators as a trait in the standard library. The trait has a method called
    // next which returns an Option<T> where T is the next value in the sequence. When the iterator
    // is finished it returns None.

    // Iterators make extensive use of closures.

    // The below example shows how to use the iterator trait to iterate over a vector.
    let v1 = vec![1, 2];
    let mut v1_iter = v1.iter(); // Returns an iterator over the vector
    assert_eq!(v1_iter.next(), Some(&1)); // Returns the first value in the vector
    assert_eq!(v1_iter.next(), Some(&2)); // Returns the second value in the vector
    assert_eq!(v1_iter.next(), None); // Returns None as there are no more values in the vector



}