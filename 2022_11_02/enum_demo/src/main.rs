#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

struct PokerCard {
    suit: PokerSuit,
    value: u8
}

fn main() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };

    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };

    println!("The value of c1: {}", c1.value);
    println!("The value of c2: {}", c2.value);

    print_suit(heart);
    print_suit(diamond);
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}