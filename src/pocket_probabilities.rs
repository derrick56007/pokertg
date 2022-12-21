use clap::Parser;

use poker::Rank;
use polars::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let df = CsvReader::from_path(args.file.to_string())
        .unwrap()
        .finish()
        .unwrap();

    // let mut file =
    //     std::fs::File::create(format!("out_{}", args.file)).unwrap();
    // CsvWriter::new(&mut file).finish(&mut df2).unwrap();

    let ranks = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '2'];
    for a in ranks {
        let mut build = "".to_string();
        for b in ranks {
            let a_r = Rank::try_from(a).unwrap();
            let b_r = Rank::try_from(b).unwrap();

            if a_r >= b_r {
                let search_string = format!("{}{}", b, a);
                let curr = df
                    .clone()
                    .lazy()
                    .filter(col("hand_rank").eq(lit(search_string.to_string())))
                    .collect()
                    .unwrap();
                let won = curr
                    .clone()
                    .lazy()
                    .filter(col("won").eq(lit(1)))
                    .filter(col("hand_rank").eq(lit(search_string)))
                    .collect()
                    .unwrap();

                let val = (won.height() as f32) / (curr.height() as f32) * 100.0;

                build += &format!(", {}{} : {:.2}", a, b, val);
            } else {
                let search_string = format!("{}{}", a, b);
                let curr = df
                    .clone()
                    .lazy()
                    .filter(col("suited").eq(lit(1)))
                    .filter(col("hand_rank").eq(lit(search_string.to_string())))
                    .collect()
                    .unwrap();
                let won = curr
                    .clone()
                    .lazy()
                    .filter(col("won").eq(lit(1)))
                    .filter(col("suited").eq(lit(1)))
                    .filter(col("hand_rank").eq(lit(search_string)))
                    .collect()
                    .unwrap();
                let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
                build += &format!(", {}{}s: {:.2}", b, a, val);
            }
        }
        println!("{}", &build[2..]);
    }
    println!("");

    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_high_card").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_high_card").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_high_card: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_pair: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_top_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_top_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_top_pair: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_top_pair_top_kicker").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_top_pair_top_kicker").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_top_pair_top_kicker: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_two_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_two_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_two_pair: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_three_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_three_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_three_of_a_kind: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_straight").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_straight").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_straight: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_flush: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_full_house").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_full_house").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_full_house: {:.2}", val);
    
    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_four_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_four_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_four_of_a_kind: {:.2}", val);
    
    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_straight_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_straight_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_straight_flush: {:.2}", val);
    
    let curr = df
        .clone()
        .lazy()
        .filter(col("flop_royal_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("flop_royal_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("flop_royal_flush: {:.2}", val);    

    println!("");

    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_high_card").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_high_card").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_high_card: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_pair: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_top_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_top_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_top_pair: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_top_pair_top_kicker").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_top_pair_top_kicker").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_top_pair_top_kicker: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_two_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_two_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_two_pair: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_three_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_three_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_three_of_a_kind: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_straight").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_straight").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_straight: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_flush: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_full_house").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_full_house").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_full_house: {:.2}", val);
    
    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_four_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_four_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_four_of_a_kind: {:.2}", val);
    
    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_straight_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_straight_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_straight_flush: {:.2}", val);
    
    let curr = df
        .clone()
        .lazy()
        .filter(col("turn_royal_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("turn_royal_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("turn_royal_flush: {:.2}", val);    

    println!("");

    let curr = df
        .clone()
        .lazy()
        .filter(col("river_high_card").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_high_card").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_high_card: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("river_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_pair: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("river_top_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_top_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_top_pair: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("river_top_pair_top_kicker").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_top_pair_top_kicker").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_top_pair_top_kicker: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("river_two_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_two_pair").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_two_pair: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("river_three_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_three_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_three_of_a_kind: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("river_straight").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_straight").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_straight: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("river_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_flush: {:.2}", val);

    let curr = df
        .clone()
        .lazy()
        .filter(col("river_full_house").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_full_house").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_full_house: {:.2}", val);
    
    let curr = df
        .clone()
        .lazy()
        .filter(col("river_four_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_four_of_a_kind").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_four_of_a_kind: {:.2}", val);
    
    let curr = df
        .clone()
        .lazy()
        .filter(col("river_straight_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_straight_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_straight_flush: {:.2}", val);
    
    let curr = df
        .clone()
        .lazy()
        .filter(col("river_royal_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let won = curr
        .clone()
        .lazy()
        .filter(col("won").eq(lit(1)))
        .filter(col("river_royal_flush").eq(lit(1)))
        .collect()
        .unwrap();
    let val = (won.height() as f32) / (curr.height() as f32) * 100.0;
    println!("river_royal_flush: {:.2}", val);    
}
