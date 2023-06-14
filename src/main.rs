
//https://www.programiz.com/rust/ownership
fn main() {
    substring_extraction();
    string_manipulation();
    mutable_slice();

    let mut s = String::from("Hello, world!");
    {
        let _substr = s;
        println!("Substring: {}", _substr);
        s = _substr;

    }
    println!("Substring: {}", s); //error: value borrowed here after move


    let mut str = String::from("hello");

    // mutable reference 1
    let ref1 = &str;

    // mutable reference 2
    let ref2 = &str[2..];

    println!("{}, {}", ref1, ref2);    

}




//A. Slices:
//1. Substring extraction: You can use slices to extract a portion of a string, array, or vector. 
//For example, given a string s, you can extract a substring using &s[start..end], 
//where start is the starting index and end is the ending index (exclusive).
fn substring_extraction() {
    ///////////////0123456789012
    let s = "Hello, World!";
    
    // Extract a substring from index 7 to 12 (exclusive)
    let _substring = &s[7..12];
    println!("Substring: {}", _substring);    
    let _substring = &s[..5];
    println!("Substring: {}", _substring);

}

fn string_manipulation() {
    let mut s = String::from("Hello, world!");
    let world_index = s.find("world").unwrap();
    s.replace_range(world_index..world_index+5, "Rust");
    println!("{}", s);  // prints: "Hello, Rust!"
}


//We can create a mutable slice by using the &mut keyword.
fn mutable_slice() {
    // mutable array
    let mut colors = ["red", "green", "yellow", "white"];
    
    println!("array = {:?}", colors);

    // mutable slice
    let mut sliced_colors = &mut colors[1..3];
    
    println!("original slice = {:?}", sliced_colors);

    // change the value of the original slice at the first index
    sliced_colors[1] = "purple";

    println!("changed slice = {:?}", sliced_colors);
    println!("colors = {:?}", colors);

    sliced_colors = &mut colors[..2];
    println!("changed slice [..2] = {:?}", sliced_colors);
    sliced_colors = &mut colors[2..];
    println!("changed slice [2..] = {:?}, {} ", sliced_colors, sliced_colors[0]);



}