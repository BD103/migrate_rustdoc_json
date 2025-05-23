mod args;
mod migrate;
mod migrations;
mod version;

fn main() {
    let args = args::parse_args().unwrap();

    let input = std::fs::read_to_string(&args.input).unwrap();

    let output = self::migrations::migrate_up(&input, args.to_version);

    println!("{output}");
}
