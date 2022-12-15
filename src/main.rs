use clap::Parser;
use bi5::*;
use anyhow::Error;
use chrono::{
    naive::NaiveDateTime,
    Duration
};

/// Command line arguments
#[derive(Parser, Debug)]
#[command(name = "catbi5")]
#[command(author = "Mayer Analytics <admin@mayeranalytics.com>")]
#[command(about = "Dump a bi5 tick file to stdout.")]
#[command(version, about, long_about = None)]
struct CliArgs {
   /// Filename
   file: String,
   /// Date in yyyy-mm-ddTHH:MM:SS format
   #[arg(short, long="date")]
   date_time: Option<NaiveDateTime>,
   /// Separator
   #[arg(short, long="sep", default_value_t=String::from("\t"))]
   sep: String
}


fn main() -> Result<(), Error> 
{
    let args: CliArgs = CliArgs::parse();
    let ticks: Vec<Tick> = read_bi5(&args.file)?;
    let sep = &args.sep;
    println!("t{}bid{}ask{}bidsize{}asksize",sep,sep,sep,sep);
    if let Some(date_time) = args.date_time {
        for tick in ticks.iter() {
            let t: NaiveDateTime = date_time + Duration::milliseconds(tick.millisecs as i64);
            println!("{}{}{}{}{}{}{}{}{}", 
                     t, sep, tick.bid, sep, tick.ask, sep, tick.bidsize, sep, tick.asksize
                    );
        }
    } else {
        for tick in ticks.iter() {
            println!("{}{}{}{}{}{}{}{}{}", 
                     tick.millisecs, sep, tick.bid, sep, tick.ask, sep, tick.bidsize, sep, tick.asksize
                    );
        }
    }
    Ok(())
}
