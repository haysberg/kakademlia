use clap::{Parser, Subcommand, crate_version};
use tokio;

#[derive(Parser, Debug)]
#[clap(name = "BigBlackChord")]
#[clap(author = "Téo Haÿs <me@haysberg.io>")]
#[clap(version = crate_version!())]
#[clap(about = "DHT Server and client with logging")]
#[clap(subcommand_required = true)]
#[clap(propagate_version = true)]
pub struct EntryArgs {
    #[clap(subcommand)]
    pub sorc: Sorc,
    #[clap(required = true, long)]
    socket: String,
}

#[derive(Subcommand, Debug)]
//Sorc == Server Or Client
pub enum Sorc {
    Server,
    Client
}



#[tokio::main]
async fn main() {
	
    
}