pub fn mutable_borrow_for_scoped_mutation() {
    let mut data = vec![1, 2, 3];

    {
        let slice = &mut data[..]; // Mutable borrow of the entire vector
        for elem in slice.iter_mut() {
            *elem *= 2; // Mutate each element
        }
    } // Mutable borrow ends here

    println!("Mutated data: {:?}", data); // Safe to use `data` again
}
