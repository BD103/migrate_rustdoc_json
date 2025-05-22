mod args;
mod migrate;
mod version;

fn main() {
    let args = args::parse_args().unwrap();

    let file_contents = std::fs::read_to_string(&args.input).unwrap();
    let current_version = self::version::detect_version(&file_contents);

    println!("{args:?} {current_version}");
}
