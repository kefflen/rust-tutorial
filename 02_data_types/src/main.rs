

fn main() {
    //Const need to have the annotation and is utils to global variables
    const _HOUR_TO_SECONDS_FATOR: u64 = 60*60;

    // Signed type begin with "i" and unsigned with "u"
    let _signed_integer: i64 = -24;
    let _unsigned_integer: u8 = 5;

    // Char is writen with single quote
    let _characterer: char = 'a';

    // Tuple can have multiple types
    let tuple: (i8, u8, char) = (-2, 4, 'c');
    let (_t1, _t2, _char) = tuple;
    let _t3 = tuple.2;

    // Use array when array have fixed size. Otherwise use vector.
    let _array: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    
}
