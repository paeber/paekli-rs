use clap::Parser;
use clap::Subcommand;


#[derive(Subcommand)]
enum Command {
    /// Send a paekli to a friend
    Send {
        /// The content of the paekli
        content: String,
    },
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
* throws your paekli directly in the system trash, hope we find it again if needed *";

const RECEIVE_MESSAGE: &str = "\
There aren't any paekli for you at the moment.
* tries to hide paekli in the trash can *";


fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli")
    .expect("the user's home directory seems to be corrupt");
    let storage_dir = project_dir.data_dir();
    std::fs::create_dir_all(storage_dir).expect("failed to create storage directory");


    match args.command {
        Command::Send { content } => {

            // check if file exists
            let file = storage_dir.join("content");
            if file.exists() {
                anyhow::bail!("no more paekli sending capacity available");
            }

            std::fs::write(storage_dir.join("content"), content)
                .expect("failed to store paekli");
            println!("{SEND_MESSAGE}");
        }
        Command::Receive => println!("{RECEIVE_MESSAGE}"),
    } 

    Ok(())

}

#[test]
#[should_panic]
fn paekli_llc_is_closed() {
    main();
}