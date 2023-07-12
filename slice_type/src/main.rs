fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item ==  b' '{
            return &s[0..i];
        } 
    
    }

    &s[..]

}

fn rust_slices(){
    println!(" Rust slices : instead of copying the entire string portion stores only the pointer to the starting index of the slice and the length of the slice, and capacity");
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world =  &s[6..11];

    println! ("{hello}, {world}")
}

fn main() {
    let s = String::from("helloworld"); //add mut since we are going to clear it later
    let word = first_word(&s);
    // s.clear(); //empties the string , makes it ""
    println! ("{word}, {s}");

    println! (" ----------------------------------------------------------------------------------------------------------------");
    println! (" Rust Slices");
    rust_slices();
}
