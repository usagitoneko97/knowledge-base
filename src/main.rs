use std::fs::read_to_string;
use structopt::StructOpt;
use crate::data::Knowledge;
use std::path::{PathBuf, Path};
use std::str::FromStr;

mod data;
mod config;

static CONFIG_FILE: &str = "kb.conf";

#[derive(StructOpt)]
struct Kb {
    #[structopt(short, long, parse(from_os_str), default_value=CONFIG_FILE)]
    config: PathBuf,
    #[structopt(subcommand)]
    cmd: KbSub
}

#[derive(StructOpt)]
enum KbSub {
    Add {
        topic: String,

        #[structopt(short, long)]
        content: String,

        #[structopt(short, long)]
        tag: String,

        #[structopt(short, long)]
        descriptions: String,
    }
}

fn init(config_file: &PathBuf) -> std::io::Result<(config::Config)> {
    // init default config location
    let config;
    config = config::Config::new(config_file)
        .expect("Error in reading config file");
    Ok(config)
}

fn main() {
    let args = Kb::from_args();
    let config = init(&args.config).unwrap();
    match args.cmd {
        KbSub::Add{topic, content, tag, descriptions} => {
            let mut d = data::Handler::new(&config);
            let knowledge = Knowledge::new(topic.clone(), content,
                                           descriptions);
            // d.add_knowledge(&knowledge);
            // knowledge.write(&config);
            d.read_all_files();
            println!("{}", d.datas.len());
            for (_, k) in d.datas.iter() {
                println!("{}", k);
            }
        }
    };
}
