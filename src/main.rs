use clap::Command;

fn main() {
    let _matches = Command::new("echor")
        .version("0.1.0")
        .author("Hedy Midassi")
        .about("Echo made with Rust")
        .get_matches();

}
