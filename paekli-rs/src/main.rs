use clap::Parser;
use clap::Subcommand;




#[derive(Subcommand)]
enum Command {
    /// Send a paekli to a friend
    Send,
    /// Receive a paekli from a friend
    Receive,
}

/// send and receive joy with ✨ paekli-cli ✨
#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}


const SEND_MESSAGE: &str = "\
Thank you for trusting Paekli LLC!
We will deliver your paekli in mint condition.
* throws your paekli directly in the trash *";

const RECEIVE_MESSAGE: &str = "\
There aren't any paekli for you at the moment.
* tries to hide paekli in the trash can *";

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Send => println!("{SEND_MESSAGE}"),
        Command::Receive => println!("{RECEIVE_MESSAGE}"),
    } 


}

#[test]
#[should_panic]
fn paekli_llc_is_closed() {
    main();
}