fn string_learning() {

    // a proper String struct
    let string = String::from("127.0.0.1:8080");

    // '&str' is a string slice, an immutable view into a string
    // By nature it's a reference/pointer
    let string_slice = &string[10..14];
    let string_borrow: &str = &string;

    // literal strings are also &str, string slices
    let string_literal = "1234";

    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);
}