use clap::Parser;
use day::{Day, Part};

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

#[derive(Debug, clap::Parser)]
#[clap(color = concolor_clap::color_choice())]
struct Cli {
    #[command(flatten)]
    color: concolor_clap::Color,

    #[arg(value_enum)]
    day: Day,

    #[arg(value_enum, default_value_t = Part::Part1)]
    part: Part,

    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let file_path = cli.file.unwrap_or(format!("inputs/{}.txt", cli.day));
    println!(">>> {}, {}", cli.day, cli.part);
    println!(">>> file: {}", file_path);

    match cli.day {
        Day::Day01 => day01::run(cli.part, &file_path),
        Day::Day02 => day02::run(cli.part, &file_path),
        Day::Day03 => day03::run(cli.part, &file_path),
        Day::Day04 => day04::run(cli.part, &file_path),
        Day::Day05 => day05::run(cli.part, &file_path),
        Day::Day06 => day06::run(cli.part, &file_path),
        Day::Day07 => day07::run(cli.part, &file_path),
        Day::Day08 => day08::run(cli.part, &file_path),
        Day::Day09 => day09::run(cli.part, &file_path),
        Day::Day10 => day10::run(cli.part, &file_path),
        Day::Day11 => day11::run(cli.part, &file_path),
    }
}
