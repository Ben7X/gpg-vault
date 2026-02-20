use clap::Parser;
use clap::Subcommand;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Debug, Parser)]
#[clap(version)]
#[command(author = "Author Name", version, about)]
pub struct Args {
    #[command(flatten)]
    pub verbosity: Verbosity<InfoLevel>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init {
        #[clap(
            short = 'd',
            long,
            help = "[false] Creates default config",
            action,
            default_value_t = false
        )]
        create_default: bool,
    },
    Status {
        filter: Vec<String>,
    },
    Config,
    Seal {
        #[clap(
            short = 'd',
            long,
            help = "[false] No commands applied",
            action,
            default_value_t = false
        )]
        dry_run: bool,

        #[clap(
            short,
            long,
            action,
            help = "[true] Shows status update at the end",
            default_value_t = true
        )]
        status: bool,

        filter: Vec<String>,
    },
    Unseal {
        #[clap(
            short = 'd',
            long,
            help = "[false] No commands applied",
            action,
            default_value_t = false
        )]
        dry_run: bool,

        #[clap(
            short,
            long,
            action,
            help = "[true] Shows status update at the end",
            default_value_t = true
        )]
        status: bool,

        filter: Vec<String>,
    },
}
