#![allow(clippy::all)]

// The following fuctions are used to provide the basic nescessities of the game with the main function at the bottom.

fn shuffle(deck: &[u8; 64]) -> [Vec<u8>; 4] {
    let mut deck1: Vec<u8> = Vec::new();
    let mut deck2: Vec<u8> = Vec::new();
    let mut deck3: Vec<u8> = Vec::new();
    let mut deck4: Vec<u8> = Vec::new();

    for a in 0..16 {
        deck4.push(deck[(a * 2) + 3]);
        deck3.push(deck[(a * 2) + 2]);
        deck2.push(deck[(a * 2) + 1]);
        deck1.push(deck[a * 2]);
    }
    println!(
        "Deck1 : {:?}    Deck2 : {:?}    Deck3 : {:?}      Deck4 : {:?}",
        deck1, deck2, deck3, deck4
    );
    [deck1, deck2, deck3, deck4]
}

// fn deal(shuf: &[u8; 64]) -> [u8; 64]
// {
//     println!("Running-----------------------------------------");

//     // let mut value = shuf.clone();
//     // let both: [Vec<u8>; 4] = shuffle(shuf, );
//     // let deck1: &Vec<u8> = & both[0];
//     // let deck2: &Vec<u8> = & both[1];
//     // let deck3: &Vec<u8> = & both[2];
//     // let deck4: &Vec<u8> = & both[3];

//     *shuf
// }