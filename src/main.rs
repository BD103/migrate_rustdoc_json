mod version;

fn main() {
    let mut args = std::env::args().skip(1);

    let file = args.next().unwrap();
    let to_version: u32 = args.next().and_then(|s| s.parse().ok()).unwrap();

    let file_contents = std::fs::read_to_string(file).unwrap();

    let current_version = self::version::detect_version(&file_contents);

    println!("{current_version} {to_version}");
}
