fn main() {
    
    // enums can contain structs or other enums
    struct Ipv4Addr {}
    struct Ipv6Addr {}
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // enums can contain fields of different data types
    enum Message {
        Quit,
        Move { x: i32, y: i32}, // ananymous struct
        Write(String),
        ChangeColor(i32, i32, i32), 
    }

    // Can also define methods for enums 
    impl Message {
        fn call(&self) {

        }
    }
    let m = Message::Write(String::from("Hello"));
    m.call();

    // Option enum can also be used when value can be something, or nothing
    // similar to 'maybe' in haskell, useful for error checking   
    // <T> syntax is likw templates in haskell or generics in java
    enum Option<T> {
        Some(T),
        None,
    }

    // None number can be either type None or i32
    // Note Option::None syntax must be used in this case because Option<T> is defined in the prelude
    //  and again in Main so the complier will throw a type mismatch without an explicit scope
    let _absent_number: Option<i32> = Option::None; 
    // Complier infers type Option<String> by defining Some
    let _some_string = Some("a string");

    // NOTE Some is not a valid value, i.e. compiler will not 
    // allow aritmetic operators ect. as this could be unsafe
    let _x: i8 = 5;
    let _y = Some(5);
    // let sum = x + y will not compile as this could
    // be unsafe in the case that y is None value and 
    // rust does all of its error checking statically at
    // compile time

    // More info on Options as they are extremly important:
    // https://doc.rust-lang.org/stable/std/option/enum.Option.html
    
    // enums are often used within match statements as they make it easy
    // to handle all possible variants of enum values, more on match in match
    // project

}
