fn main() {
    // if let statements useful to write clean/consise code in the case
    // that a values matches one pattern and nothing else
    let some_u8 = Some(0u8);
    match some_u8 {
        Some(3) => println!("Three"),
        _ => (),
    }

    // This match statement contains a lot of boilerplate code to match
    // only a single pattern, this can be written in more consisly with a if let
    let some_u8_val = Some(0u8);
    if let Some(3) = some_u8_val {
        println!("Three");
    }

    // if let works the same as a match statment in that it attempts to match a single pattern 
    // will ignore all other values of Some()
    // if let is a syntactic sugar on top of match statement, however, the exaustive property 
    // enforced in match statements in not enforced in if let so the programmer must be careful 
    // without that assurance 

    // an else statement can also be included in an if let statement and is a syntatic sugar
    // to cover the  _ => () case in a match statement
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // the rest of the states
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    // The following 2 code blocks are equivalent 
    let mut count = 0;
    let coin = Coin::Dime;

    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let coin = Coin::Dime;
    let mut count_w_else = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count_w_else += 1;
    }

}
