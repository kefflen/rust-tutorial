

fn main() {
    //Const need to have the annotation and is utils to global variables
    const HOUR_TO_SECONDS_FATOR: u64 = 60*60;

    // Signed type begin with "i" and unsigned with "u"
    let signed_integer: i64 = -24;
    let unsigned_integer: u8 = 5;

    // Char is writen with single quote
    let characterer: char = 'a';

    // Tuple can have multiple types
    let tuple: (i8, u8, char) = (-2, 4, 'c');
    let (t1, t2, char) = tuple;
    let t3 = tuple.2;

    // Use array when array have fixed size. Otherwise use vector.
    let array: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    
}
