fn main() {
    // String::from allocates on heap, as size is unknown etc etc
    // Rust will free s1 when it is out of scope automagically
    let mut s1 = String::from("yo");
    s1.push_str(" lol");
    println!("{}", s1);

    // by doing this, rust considers s1 no longer valid, so we don't accidentally free the same bit of memory twice
    // this is called a _move_
    // so we're not doing a copy here, simply moving the reference to the stuff on the heap
    let s2 = s1;
    // println!("{}", s1); <- so this won't compile
    println!("{}", s2);

    // if we do want to copy, we can clone (this is nice vs other langs as it's explicit, potentially expensive)
    let s3 = s2.clone();
    println!("s2 = {0}, s3 = {1}", s2, s3);

    // ownership also happens across functions, so once we call take ownership we no longer have access to s3
    take_ownership(s3);
    // println!("s3 after ownership {}", s3); // error[E0382]: borrow of moved value: `s3`

    // referencing and borrowing
    let s4 = String::from("lets get referencing and borrowing");
    borrow(&s4); // & for ref to a thing
    println!("i still own s4 coz i can print it {}", s4);


    let mut s5 = String::from("mutable bootable");
    // let s6 = &mut s5; //can only have one mut ref per scope. Prevents data races
    mutable_borrow(&mut s5);
    println!("{0}", s5)
}

fn take_ownership(x: String) {
    println!("haha i own you {}", x);
}

// taking a reference is called borrowing
fn borrow(x: &String) {
    // x.push_str("poop"); // references are immutable by default
    println!("i don't own you {}", x)
}

fn mutable_borrow(x: &mut String) {
    x.push_str("poop")
}