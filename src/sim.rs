extern crate pbr;

use clap::Parser;
use pbr::ProgressBar;
use poker::{Card, Eval, Evaluator, Rank};
use polars::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of rounds
    #[arg(short, long, default_value_t = 2_000_000)]
    rounds: usize,

    /// Number of players
    #[arg(short, long, default_value_t = 4)]
    players: usize,
}

fn main() {
    let args = Args::parse();
    let rows: usize = args.rounds * args.players;

    let eval = Evaluator::new();

    let mut deal_ids: Vec<u32> = vec![0; rows];
    let mut boards: Vec<String> = vec![String::new(); rows];
    let mut sorted_flops: Vec<String> = vec![String::new(); rows];
    let mut sorted_turns: Vec<String> = vec![String::new(); rows];
    let mut sorted_rivers: Vec<String> = vec![String::new(); rows];
    let mut hands: Vec<String> = vec![String::new(); rows];
    let mut won: Vec<bool> = vec![false; rows];

    let mut hand_ranks: Vec<String> = vec![String::new(); rows];
    let mut pockets: Vec<bool> = vec![false; rows];
    let mut suited: Vec<bool> = vec![false; rows];
    let mut connectors: Vec<bool> = vec![false; rows];

    let mut connectors_1_gap: Vec<bool> = vec![false; rows];
    let mut connectors_2_gap: Vec<bool> = vec![false; rows];
    let mut connectors_3_gap: Vec<bool> = vec![false; rows];
    let mut connectors_4_gap: Vec<bool> = vec![false; rows];
    let mut connectors_5_gap: Vec<bool> = vec![false; rows];

    let mut flop_high_card: Vec<bool> = vec![false; rows];
    let mut flop_pair: Vec<bool> = vec![false; rows];
    let mut flop_two_pair: Vec<bool> = vec![false; rows];
    let mut flop_three_of_a_kind: Vec<bool> = vec![false; rows];
    let mut flop_straight: Vec<bool> = vec![false; rows];
    let mut flop_flush: Vec<bool> = vec![false; rows];
    let mut flop_full_house: Vec<bool> = vec![false; rows];
    let mut flop_four_of_a_kind: Vec<bool> = vec![false; rows];
    let mut flop_straight_flush: Vec<bool> = vec![false; rows];
    let mut flop_royal_flush: Vec<bool> = vec![false; rows];

    let mut turn_high_card: Vec<bool> = vec![false; rows];
    let mut turn_pair: Vec<bool> = vec![false; rows];
    let mut turn_two_pair: Vec<bool> = vec![false; rows];
    let mut turn_three_of_a_kind: Vec<bool> = vec![false; rows];
    let mut turn_straight: Vec<bool> = vec![false; rows];
    let mut turn_flush: Vec<bool> = vec![false; rows];
    let mut turn_full_house: Vec<bool> = vec![false; rows];
    let mut turn_four_of_a_kind: Vec<bool> = vec![false; rows];
    let mut turn_straight_flush: Vec<bool> = vec![false; rows];
    let mut turn_royal_flush: Vec<bool> = vec![false; rows];

    let mut river_high_card: Vec<bool> = vec![false; rows];
    let mut river_pair: Vec<bool> = vec![false; rows];
    let mut river_two_pair: Vec<bool> = vec![false; rows];
    let mut river_three_of_a_kind: Vec<bool> = vec![false; rows];
    let mut river_straight: Vec<bool> = vec![false; rows];
    let mut river_flush: Vec<bool> = vec![false; rows];
    let mut river_full_house: Vec<bool> = vec![false; rows];
    let mut river_four_of_a_kind: Vec<bool> = vec![false; rows];
    let mut river_straight_flush: Vec<bool> = vec![false; rows];
    let mut river_royal_flush: Vec<bool> = vec![false; rows];

    let mut pb = ProgressBar::new(args.rounds as u64);
    pb.format("[##-]");

    for deal_id in 0..args.rounds {
        let mut deck = Card::generate_shuffled_deck().to_vec();
        let board: Vec<Card> = deck.drain(..5).collect();

        let flop = &board[..3];
        let turn = &board[..4];
        let river = &board[..5];

        let mut sorted_flop = flop.to_vec();
        sorted_flop.sort();
        let mut sorted_turn = turn.to_vec();
        sorted_turn.sort();
        let mut sorted_river = river.to_vec();
        sorted_river.sort();

        let mut current_hands: Vec<Vec<Card>> = Vec::new();

        let mut best: Eval = eval.evaluate(river).unwrap();

        for hand_id in 0..args.players {
            let mut hand: Vec<Card> = deck.drain(..2).collect();
            hand.sort();

            let eval_res = eval.evaluate([river, &hand].concat()).unwrap();
            let idx: usize = deal_id * args.players + hand_id;

            if hand_id == 0 {
                best = eval_res;
                won[idx] = true;
            } else if eval_res.is_equal_to(best) {
                won[idx] = true;
            } else if eval_res.is_better_than(best) {
                won[(deal_id * args.players)..((deal_id + 1) * args.players)].fill(false);
                won[idx] = true;
                best = eval_res;
            }

            // <pre>,hand,board,flop Straight Flush,turn Straight Flush,river Straight Flush,flop %,turn %,river %,flop Four of a Kind,turn Four of a Kind,river Four of a Kind,flop Full House,turn Full House,river Full House,flop Flush,turn Flush,river Flush,flop Straight,turn Straight,river Straight,flop Three of a Kind,turn Three of a Kind,river Three of a Kind,flop Two Pair,turn Two Pair,river Two Pair,flop Pair,turn Pair,river Pair,flop High Card,turn High Card,river High Card,won</pre>
            deal_ids[idx] = deal_id as u32;
            boards[idx] = format!(
                "{} {} {} {} {}",
                board[0].rank_suit_string(),
                board[1].rank_suit_string(),
                board[2].rank_suit_string(),
                board[3].rank_suit_string(),
                board[4].rank_suit_string()
            );
            sorted_flops[idx] = format!(
                "{} {} {}",
                sorted_flop[0].rank_suit_string(),
                sorted_flop[1].rank_suit_string(),
                sorted_flop[2].rank_suit_string(),
            );
            sorted_turns[idx] = format!(
                "{} {} {} {}",
                sorted_turn[0].rank_suit_string(),
                sorted_turn[1].rank_suit_string(),
                sorted_turn[2].rank_suit_string(),
                sorted_turn[3].rank_suit_string(),
            );
            sorted_rivers[idx] = format!(
                "{} {} {} {} {}",
                sorted_river[0].rank_suit_string(),
                sorted_river[1].rank_suit_string(),
                sorted_river[2].rank_suit_string(),
                sorted_river[3].rank_suit_string(),
                sorted_river[4].rank_suit_string(),
            );

            hands[idx] = format!(
                "{} {}",
                hand[0].rank_suit_string(),
                hand[1].rank_suit_string()
            );
            let flop_eval = eval
                .evaluate([flop, &hand].concat())
                .expect("Couldn't evaluate hand!");

            hand_ranks[idx] = format!("{}{}", hand[0].rank().as_char(), hand[1].rank().as_char());
            pockets[idx] = hand[0].rank() == hand[1].rank();
            suited[idx] = hand[0].suit() == hand[1].suit();
            connectors[idx] = if pockets[idx] {
                false
            } else {
                match hand[1].rank() {
                    Rank::Ace => hand[0].rank() == Rank::King || hand[0].rank() == Rank::Two,
                    _ => hand[1].rank() as i32 - hand[0].rank() as i32 == 1,
                }
            };
            connectors_1_gap[idx] = if pockets[idx] {
                false
            } else {
                match hand[1].rank() {
                    Rank::Ace => hand[0].rank() == Rank::Queen || hand[0].rank() == Rank::Three,
                    _ => hand[1].rank() as i32 - hand[0].rank() as i32 == 2,
                }
            };
            connectors_2_gap[idx] = if pockets[idx] {
                false
            } else {
                match hand[1].rank() {
                    Rank::Ace => hand[0].rank() == Rank::Jack || hand[0].rank() == Rank::Four,
                    _ => hand[1].rank() as i32 - hand[0].rank() as i32 == 3,
                }
            };
            connectors_3_gap[idx] = if pockets[idx] {
                false
            } else {
                match hand[1].rank() {
                    Rank::Ace => hand[0].rank() == Rank::Ten || hand[0].rank() == Rank::Five,
                    _ => hand[1].rank() as i32 - hand[0].rank() as i32 == 4,
                }
            };
            connectors_4_gap[idx] = if pockets[idx] {
                false
            } else {
                match hand[1].rank() {
                    Rank::Ace => hand[0].rank() == Rank::Nine || hand[0].rank() == Rank::Six,
                    _ => hand[1].rank() as i32 - hand[0].rank() as i32 == 5,
                }
            };
            connectors_5_gap[idx] = if pockets[idx] {
                false
            } else {
                match hand[1].rank() {
                    Rank::Ace => hand[0].rank() == Rank::Eight || hand[0].rank() == Rank::Seven,
                    _ => hand[1].rank() as i32 - hand[0].rank() as i32 == 6,
                }
            };

            flop_high_card[idx] = flop_eval.is_high_card();
            flop_pair[idx] = flop_eval.is_pair();
            flop_two_pair[idx] = flop_eval.is_two_pair();
            flop_three_of_a_kind[idx] = flop_eval.is_three_of_a_kind();
            flop_straight[idx] = flop_eval.is_straight();
            flop_flush[idx] = flop_eval.is_flush();
            flop_full_house[idx] = flop_eval.is_full_house();
            flop_four_of_a_kind[idx] = flop_eval.is_four_of_a_kind();
            flop_straight_flush[idx] = flop_eval.is_straight_flush();
            flop_royal_flush[idx] = flop_eval.is_royal_flush();

            let turn_eval = eval
                .evaluate([turn, &hand].concat())
                .expect("Couldn't evaluate hand!");

            turn_high_card[idx] = turn_eval.is_high_card();
            turn_pair[idx] = turn_eval.is_pair();
            turn_two_pair[idx] = turn_eval.is_two_pair();
            turn_three_of_a_kind[idx] = turn_eval.is_three_of_a_kind();
            turn_straight[idx] = turn_eval.is_straight();
            turn_flush[idx] = turn_eval.is_flush();
            turn_full_house[idx] = turn_eval.is_full_house();
            turn_four_of_a_kind[idx] = turn_eval.is_four_of_a_kind();
            turn_straight_flush[idx] = turn_eval.is_straight_flush();
            turn_royal_flush[idx] = turn_eval.is_royal_flush();

            let river_eval = eval
                .evaluate([river, &hand].concat())
                .expect("Couldn't evaluate hand!");

            river_high_card[idx] = river_eval.is_high_card();
            river_pair[idx] = river_eval.is_pair();
            river_two_pair[idx] = river_eval.is_two_pair();
            river_three_of_a_kind[idx] = river_eval.is_three_of_a_kind();
            river_straight[idx] = river_eval.is_straight();
            river_flush[idx] = river_eval.is_flush();
            river_full_house[idx] = river_eval.is_full_house();
            river_four_of_a_kind[idx] = river_eval.is_four_of_a_kind();
            river_straight_flush[idx] = river_eval.is_straight_flush();
            river_royal_flush[idx] = river_eval.is_royal_flush();

            // hand[0].ra
            current_hands.push(hand);
        }

        pb.inc();
    }

    println!("writing to file..");

    let mut df = df!(
        "deal_id" => deal_ids,
        "board" => boards,
        "sorted_flop" => sorted_flops,
        "sorted_turn" => sorted_turns,
        "sorted_river" => sorted_rivers,
        "hand" => hands,
        "won" => won.iter().map(|&f| f as i32).collect::<Vec<i32>>(),

        "hand_rank" => hand_ranks,
        "pockets" => pockets.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "suited" => suited.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "connector" => connectors.iter().map(|&f| f as i32).collect::<Vec<i32>>(),

        "connector_1_gap" => connectors_1_gap.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "connector_2_gap" => connectors_2_gap.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "connector_3_gap" => connectors_3_gap.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "connector_4_gap" => connectors_4_gap.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "connector_5_gap" => connectors_5_gap.iter().map(|&f| f as i32).collect::<Vec<i32>>(),

        "flop_high_card" => flop_high_card.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "flop_pair" => flop_pair.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "flop_two_pair" => flop_two_pair.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "flop_three_of_a_kind" => flop_three_of_a_kind.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "flop_straight" => flop_straight.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "flop_flush" => flop_flush.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "flop_full_house" => flop_full_house.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "flop_four_of_a_kind" => flop_four_of_a_kind.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "flop_straight_flush" => flop_straight_flush.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "flop_royal_flush" => flop_royal_flush.iter().map(|&f| f as i32).collect::<Vec<i32>>(),

        "turn_high_card" => turn_high_card.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "turn_pair" => turn_pair.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "turn_two_pair" => turn_two_pair.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "turn_three_of_a_kind" => turn_three_of_a_kind.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "turn_straight" => turn_straight.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "turn_flush" => turn_flush.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "turn_full_house" => turn_full_house.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "turn_four_of_a_kind" => turn_four_of_a_kind.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "turn_straight_flush" => turn_straight_flush.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "turn_royal_flush" => turn_royal_flush.iter().map(|&f| f as i32).collect::<Vec<i32>>(),

        "river_high_card" => river_high_card.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "river_pair" => river_pair.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "river_two_pair" => river_two_pair.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "river_three_of_a_kind" => river_three_of_a_kind.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "river_straight" => river_straight.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "river_flush" => river_flush.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "river_full_house" => river_full_house.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "river_four_of_a_kind" => river_four_of_a_kind.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "river_straight_flush" => river_straight_flush.iter().map(|&f| f as i32).collect::<Vec<i32>>(),
        "river_royal_flush" => river_royal_flush.iter().map(|&f| f as i32).collect::<Vec<i32>>(),

    )
    .unwrap();

    let mut file =
        std::fs::File::create(format!("poker_n{}_p{}.csv", args.rounds, args.players)).unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();
}
