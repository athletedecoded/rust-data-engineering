/* 
A Polars CLI tool that uses clap to parse the following optional arguments:
* datapath: path to a csv or json file
*/

use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Kahlia Hogg",
    about = "A CLI tool for EDA using Polars",
    after_help = "Example: cargo run csv --path <path/to/data.csv> --has_headers"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Csv {
        #[clap(long)]
        path: String,
        #[clap(long)]
        has_headers: bool
    },
    // Json {
    //     #[clap(long)]
    //     path: String
    // }
}


fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Csv { path, has_headers }) => {
            println!("Loading CSV to dataframe...");
            let df = polars_eda::read_csv(&path, has_headers);
            match df {
                Ok(df) => polars_eda::df_summary(df),
                Err(e) => println!("Error: {:?}", e)
            }
        },
        // Some(Commands::Json { path }) => {
        //     println!("Loading JSON to dataframe...");
        //     let df = polars_eda::load_json(&path);
        // }
        None => println!("No command specified")
    }
}
