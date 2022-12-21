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
                build += &format!(", {}{}s: {:.2}", b, a, val);
            }
        }
        println!("{}", &build[2..]);
    }
}
