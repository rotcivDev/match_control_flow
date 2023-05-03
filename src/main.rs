fn main() {
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn plus_one(coin: Coin) {
        if let Coin::Quarter(state) = coin {
            println!("+1 State quarter from {:?}!", state);
        } else {
            println!("+1 {:?}!", coin);
        }
    }
    println!("The match Control Flow Construct");
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    println!("{}", value_in_cents(Coin::Penny));
    plus_one(Coin::Nickel);
    plus_one(Coin::Dime);
    plus_one(Coin::Quarter(UsState::Alabama));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {
        println!("add_fancy_hat");
    }
    fn remove_fancy_hat() {
        println!("remove_fancy_hat");
    }
    fn move_player(num_spaces: u8) {
        println!("move_player {}", num_spaces);
    }
}
