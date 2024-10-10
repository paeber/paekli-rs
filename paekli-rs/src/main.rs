use clap::Parser;

/// send and receive joy with ✨ paekli-cli ✨
#[derive(Parser)]
#[clap(version)]
struct Cli;

fn main() {
    let _args = Cli::parse();
    println!("Paekli LLC is currentli closed 😢");
}

#[test]
#[should_panic]
fn paekli_llc_is_closed() {
    main();
}