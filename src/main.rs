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
   input: String,
   /// Date in yyyy-mm-ddTHH:MM:SS format
   #[arg(short, long="date")]
   date_time: Option<NaiveDateTime>,
   /// Separator
   #[arg(short, long="sep", default_value_t=String::from("\t"))]
   sep: String,
   /// Count ticks
   #[arg(short, long, default_value_t=false)]
   count: bool
}


fn main() -> Result<(), Error> 
{
    let args: CliArgs = CliArgs::parse();

    let bi5 = Bi5::new(&args.input, args.date_time);

    if args.count {
        println!("{}:{}", args.input, bi5.iter()?.count());
        return Ok(())
    }

    let sep = &args.sep;
    println!("t{}bid{}ask{}bidsize{}asksize",sep,sep,sep,sep);
    for (date_time, tick) in bi5.iter()? {
        let t: NaiveDateTime = date_time + Duration::milliseconds(tick.millisecs as i64);
        println!("{}{}{}{}{}{}{}{}{}", 
                    t, sep, tick.bid, sep, tick.ask, sep, tick.bidsize, sep, tick.asksize
                );
    }
    Ok(())
}
