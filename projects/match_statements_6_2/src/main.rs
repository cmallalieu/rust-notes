fn main() {
    // match statements compare a value against a series of patterns
    // and executeds code based on the matching value
    // compiler will ensure that all possible cases are handled at compile time

    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    // takes in a Coin enum and based on the type of the 
    // Coin instance will return correct value of coin 
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny =>  { 
                println!("Lucky Penny");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    // match statements are also helpful in extracting values 
    // from enum variants

    #[derive(Debug)] // to inspect state
    enum UsState {
        Alabama,
        Alaska,
        // the rest of the states
    }
    enum CoinWithData {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents_w_data(coin: CoinWithData) -> u8 {
        match coin {
            CoinWithData::Penny => 1,
            CoinWithData::Nickel => 5,
            CoinWithData::Dime => 10,
            CoinWithData::Quarter(state) => {
                println!("Quarter with state {:?}!", state);
                25
            }
        }
    }

    // match statements are also helpful with dealing with Option<T>
    // easy to write correct code because compiler will complain at compile time
    // if all possible values of Option<T> are not handled
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let _six  = plus_one(five);
    let _none = plus_one(None);

    // matches are enforced by the compiler to be exhaustive
    // compiler will throw error at compile time if it is not
    // the following function will not compiler because it 
    // does not handle the case of Option being of None type

    // fn plus_one_error(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }

    // The _ (wildcard) placeholder pattern represents all possible
    // cases, i.e. will match all possile value. This is to satisfy the exaustive property of match 
    // statements if the domain is large and only a small subset of 
    // the domain is interesting

    // Here we only want to print the value if it is 1-5 but we must also 
    // account for all the other possible values we dont care about in order
    // to satisfy the exaustive property
    let some_u32: u32 = 5;
    match some_u32 {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => (),
    }


}
