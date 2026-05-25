use clap::{
    Parser, Subcommand
};


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct DaisyCli{

    #[clap(subcommand)]
    pub entity_type: Commands,

}



#[derive(Debug, Parser)]
pub struct StartArgs{

    ///Set port
    #[arg(short, long)]
    pub port: Option<u16>,
    #[arg(short, long)]
    pub url: Option<String>
}

#[derive(Subcommand, Debug)]
pub enum Commands{
    ///Start the server
    Start(StartArgs),
    /// Stop the server
    Stop
}