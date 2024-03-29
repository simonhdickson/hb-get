use std::{env, error::Error};

use clap::Parser;
use hb_api::HBClient;

use crate::config::Settings;

mod config;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    ListOrders,
    DownloadAll,
    DownloadOrder { gamekey: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    let config = Settings::new()?;

    let path = env::current_dir()?;

    let client = HBClient::new(path, config.headers, config.platforms);

    match opts.subcmd {
        SubCommand::ListOrders => {
            let order_items = client.list_orders().await?;

            println!("{:?}", order_items);
        }
        SubCommand::DownloadAll => {
            let order_items = client.list_orders().await?;

            for order_item in order_items {
                println!("downloading order {}", &order_item.gamekey);

                let order = client.get_order(&order_item.gamekey).await?;
                client.download_order(&order).await?;
            }
        }
        SubCommand::DownloadOrder { gamekey } => {
            let order = client.get_order(&gamekey).await?;
            println!("{:?}", order);
            client.download_order(&order).await?;
        }
    }

    Ok(())
}
