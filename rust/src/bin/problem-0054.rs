/// Problem 54
/// In the card game poker, a hand consists of five cards and are ranked, from 
/// lowest to highest, in the following way:
/// 
/// * High Card: Highest value card.
/// * One Pair: Two cards of the same value.
/// * Two Pairs: Two different pairs.
/// * Three of a Kind: Three cards of the same value.
/// * Straight: All cards are consecutive values.
/// * Flush: All cards of the same suit.
/// * Full House: Three of a kind and a pair.
/// * Four of a Kind: Four cards of the same value.
/// * Straight Flush: All cards are consecutive values of same suit.
/// * Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
/// 
/// The cards are valued in the order:
/// 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
/// 
/// If two players have the same ranked hands then the rank made up of the 
/// highest value wins; for example, a pair of eights beats a pair of fives 
/// (see example 1 below). But if two ranks tie, for example, both players have 
/// a pair of queens, then highest cards in each hand are compared (see example 
/// 4 below); if the highest cards tie then the next highest cards are compared, 
/// and so on.
/// 
/// Consider the following five hands dealt to two players:
/// 
/// Hand  Player 1           Player 2             Winner
/// ----  --------           --------             ------
/// 1     5H 5C 6S 7S KD     2C 3S 8S 8D TD       Player 2
///       Pair of Fives      Pair of Eights 
///
/// 2     5D 8C 9S JS AC     2C 5C 7D 8S QH       Player 1
///       Highest card Ace   Highest card Queen
///      
/// 3     2D 9C AS AH AC     3D 6D 7D TD QD       Player 2
///       Three Aces         Flush with Diamonds
///      
/// 4     4D 6S 9H QH QC     3D 6D 7H QD QS       Player 1
///       Pair of Queens     Pair of Queens
///       Highest card Nine  Highest card Seven
///      
/// 5     2H 2D 4C 4D 4S     3C 3D 3S 9S 9D       Player 1
///       Full House         Full House
///       With Three Fours   with Three Threes
///      
/// The file, poker.txt, contains one-thousand random hands dealt to two 
/// players. Each line of the file contains ten cards (separated by a single 
/// space): the first five are Player 1's cards and the last five are Player 2's 
/// cards. You can assume that all hands are valid (no invalid characters or 
/// repeated cards), each player's hand is in no specific order, and in each 
/// hand there is a clear winner.
/// 
/// How many hands does Player 1 win?
fn main() {
    let data: String = "\
        5H 5C 6S 6S KD 2C 2S 8S 8D TD\n\
        5H 5C 6S 7S KD 2C 3S 8S 8D TD\n\
        5D 8C 9S JS AC 2C 5C 7D 8S QH\n\
        2D 9C AS AH AC 3D 6D 7D TD QD\n\
        4D 6S 9H QH QC 3D 6D 7H QD QS\n\
        TD JS QH KH AC 3D 6D 7D QD QD\n\
        TD JD QD KD AD 3D 4D 5D 6D 7D\n\
        4D 4S 4H 4H AC 3D 3D 3D QD QD\n\
        2H 2D 4C 4D 4S 3C 3D 3S 9S 9D\n".to_string();

    let mut wins: Vec<u32> = vec![0, 0];

    for row in data.lines() {
        let mut hands: Vec<Vec<(u8,u8)>> = Vec::new();
        hands.push(Vec::new());
        hands.push(Vec::new());
        for cardstr in row.split(' ') {
            let mut card: (u8,u8) = (0,0);
            for (i,s) in cardstr.chars().enumerate() {
                match i {
                    0 => card.0 = match s {
                        '2' => 2,
                        '3' => 3,
                        '4' => 4,
                        '5' => 5,
                        '6' => 6,
                        '7' => 7,
                        '8' => 8,
                        '9' => 9,
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => 0,
                    },
                    1 => card.1 = match s {
                        'D' => 1,
                        'C' => 2,
                        'H' => 3,
                        'S' => 4,
                        _ => 0,
                    },
                    _ => (),
                };
            }
            if hands[0].len() < 5 {
                hands[0].push(card)
            } else {
                hands[1].push(card)
            }
        }
        for hand in &mut hands {
            hand.sort_by_key(|k| k.0);
        }

        println!("P1: {:?} -> {:?} \nP2: {:?} -> {:?}\n", 
            hands[0], score(&hands[0]), hands[1], score(&hands[1]));
    }
    println!("Answer: {}", wins[0]);
}

struct HandInfo {
    flush: bool,
    straight: bool,
    pairing: Pairing
}

impl HandInfo {
    fn new(hand: &Vec<(u8,u8)>) -> HandInfo {
        HandInfo {
            flush: is_flush(hand),
            straight: is_straight(hand),
            pairing: hand_pairing(hand)
        }
    }
}

enum Pairing {
    Nothing { rank: Vec<u8> },
    Pair { pair: u8, kickers: Vec<u8> },
    TwoPair { high_pair: u8, low_pair: u8, kicker: u8 },
    Three { high: u8, kickers: Vec<u8> },
    FullHouse { triplet: u8, pair: u8 },
    Four { quad: u8, kicker: u8 },
}

fn hand_pairing(hand: &Vec<(u8,u8)>) -> Pairing {
    let mut counts: Vec<u8> = vec![0; 15];
    for card in hand {
        counts[card.0 as usize] += 1;
    }
    println!("{:?}", counts);
    let max: u8 = *counts.iter().max().unwrap();
    let ind_max: u8 = counts.iter().position(|&x| x == max).unwrap() as u8;
    match max {
        4 => {
            Pairing::Four { 
                quad: ind_max, 
                kicker: counts.iter().position(|&x| x == 1).unwrap() as u8
            }
        },
        3 => {
            // Look for full house or plain three-of-a-kind
            match counts.iter().position(|&x| x == 2) {
                Some(x) => {
                    Pairing::FullHouse { 
                        triplet: ind_max, 
                        pair: x as u8 
                    }
                },
                None => {
                    Pairing::Three { 
                        high: ind_max, 
                        kickers: vec![0] 
                    }
                }
            }
        },
        2 => {
            // Look for two pair
            let num_two: u8 = counts.iter()
                .filter(|&x| *x == 2)
                .fold(0, |a, _| a + 1) as u8;

            match num_two {
                2 => {
                    Pairing::TwoPair { high_pair: 0, low_pair: 0, kicker: 0 }
                },
                1 => {
                    Pairing::Pair { pair: 0, kickers: vec![0] }
                },
                _ => {
                    Pairing::Nothing { rank: vec![0] }
                }
            }

        },
        1 => {
            let mut r: Vec<u8> = Vec::new();
            for card in hand.iter().rev() {
                r.push(card.0);
            }
            Pairing::Nothing { rank: r }
        },
        _ => Pairing::Nothing { rank: vec![0] }
    }
}

/// Compute a score for a hand, in the form of (rank, high card, breaker). 
/// The ranks are as follows:
/// 0 High Card: Highest value card.
/// 1 One Pair: Two cards of the same value.
/// 2 Two Pairs: Two different pairs.
/// 3 Three of a Kind: Three cards of the same value.
/// 4 Straight: All cards are consecutive values.
/// 5 Flush: All cards of the same suit.
/// 6 Full House: Three of a kind and a pair.
/// 7 Four of a Kind: Four cards of the same value.
/// 8 Straight Flush: All cards are consecutive values of same suit.
///   Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
/// 
/// Comparing hands of the same rank breaks ties on the high card first and then
/// on the breaker. For instance, the high card of a full house is the 
/// three-of-a-kind, and the breaker is the pair.
///
/// Suits count as equal rank, and are not used to break ties.
fn score(hand: &Vec<(u8,u8)>) -> Vec<u8> {
    let info = HandInfo::new(hand);
    if info.straight && info.flush {
        vec![8, hand[4].0]
    } else if info.flush {
        let mut ret: Vec<u8> = vec![5];
        for card in hand.iter().rev() {
            ret.push(card.0);
        }
        ret
    } else if info.straight {
        vec![4, hand[4].0]
    } else {
        match info.pairing {
            Pairing::Four { quad: q, kicker: k } => vec![7, q, k],
            Pairing::FullHouse { triplet: t, pair: p } => vec![6, t, p],
            Pairing::Three { high: h, kickers: k } => {
                let mut v: Vec<u8> = vec![3, h];
                v.extend_from_slice(&k);
                v
            },
            Pairing::TwoPair { 
                high_pair: h, low_pair: l, kicker: k 
            } => vec![2, h, l, k],
            Pairing::Pair { pair: p, kickers: k } => {
                let mut v: Vec<u8> = vec![1, p];
                v.extend_from_slice(&k);
                v
            },
            Pairing::Nothing { rank: r } => {
                let mut v: Vec<u8> = vec![0];
                // let mut k: Vec<u8> = r;
                // k.reverse();
                v.extend_from_slice(&r);
                v
            },
        }
    }
}

fn is_flush(hand: &Vec<(u8,u8)>) -> bool {
    let last: u8 = hand[0].1;
    for card in hand {
        if card.1 != last {
            return false
        }
    }
    true
}

fn is_straight(hand: &Vec<(u8,u8)>) -> bool {
    for i in 1..hand.len() {
        if hand[i].0 != hand[i-1].0+1 {
            return false
        }
    }
    true
}
