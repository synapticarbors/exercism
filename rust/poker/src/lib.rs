use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

impl FromStr for Suit {
    type Err = ();

    fn from_str(input: &str) -> Result<Suit, Self::Err> {
        match input {
            "C" => Ok(Suit::Club),
            "S" => Ok(Suit::Spade),
            "H" => Ok(Suit::Heart),
            "D" => Ok(Suit::Diamond),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Ord, Eq, PartialEq, PartialOrd)]
enum Rank {
    HighCard(u8, u8, u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    TwoPairs(u8, u8, u8),
    ThreeOfAKind(u8, u8, u8),
    Straight(u8),
    Flush(u8, u8, u8, u8, u8),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8),
}

#[derive(Debug)]
struct Card(u8, Suit);

impl Card {
    pub fn from_str(s: &str) -> Card {
        let mut it = s.chars().rev();
        let suit = it.next().unwrap();
        let v: String = it.rev().collect::<String>();

        Card(
            Card::value_from_string(&v),
            Suit::from_str(&suit.to_string()).unwrap(),
        )
    }

    fn value_from_string(s: &str) -> u8 {
        match s {
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => s.parse::<u8>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Hand<'a> {
    cards: Vec<Card>,
    rep: &'a str,
    rank: Rank,
}

impl<'a> Hand<'a> {
    pub fn from_str(s: &'a str) -> Hand {
        let mut c: Vec<Card> = s
            .split_ascii_whitespace()
            .map(|s| Card::from_str(s))
            .collect();

        c.sort_unstable_by_key(|x| -(x.0 as i8));

        let r = Hand::get_rank(&c);
        Hand {
            cards: c,
            rep: s,
            rank: r,
        }
    }

    fn group_cards(cards: &[Card]) -> Vec<(u8, u8)> {
        let mut h: HashMap<u8, u8> = HashMap::new();
        for c in cards.iter() {
            *h.entry(c.0).or_default() += 1;
        }

        let mut v: Vec<(u8, u8)> = h.iter().map(|(k, v)| (*v, *k)).collect();
        v.sort_unstable();

        v.into_iter().rev().collect()
    }

    fn get_rank(cards: &[Card]) -> Rank {
        let is_straight =
            cards.windows(2).all(|w| w[0].0 == w[1].0 + 1) || (cards[0].0 == 14 && cards[1].0 == 5);
        let is_flush = cards.iter().all(|x| x.1 == cards[0].1);

        if is_straight && is_flush {
            return Rank::StraightFlush(cards[0].0);
        }

        let gcards = Hand::group_cards(cards);

        match gcards[..] {
            [(4, v1), (1, v2)] => Rank::FourOfAKind(v1, v2),
            [(3, v1), (2, v2)] => Rank::FullHouse(v1, v2),
            [(1, v1), (1, v2), (1, v3), (1, v4), (1, v5)] if is_flush => {
                Rank::Flush(v1, v2, v3, v4, v5)
            }
            [(1, v1), (1, v2), ..] if is_straight && v1 == 14 && v2 == 5 => Rank::Straight(v2),
            [(1, v1), ..] if is_straight => Rank::Straight(v1),
            [(3, v1), (1, v2), (1, v3)] => Rank::ThreeOfAKind(v1, v2, v3),
            [(2, v1), (2, v2), (1, v3)] => Rank::TwoPairs(v1, v2, v3),
            [(2, v1), (1, v2), (1, v3), (1, v4)] => Rank::OnePair(v1, v2, v3, v4),
            [(1, v1), (1, v2), (1, v3), (1, v4), (1, v5)] => Rank::HighCard(v1, v2, v3, v4, v5),
            _ => unreachable!(),
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let game: Vec<Hand> = hands.iter().map(|s| Hand::from_str(s)).collect();
    let max_rank = game.iter().map(|h| &h.rank).max();

    game.iter()
        .filter_map(|h| {
            if Some(&h.rank) == max_rank {
                Some(Some(h.rep))
            } else {
                None
            }
        })
        .collect()
}
