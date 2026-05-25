mod daisy_help;
mod daisy_cli;
mod daisy_conf;
mod daisy_helpers;
mod daisy_dr;


//======================================================================================================================
//======================================================================================================================


use clap::{ Parser };
use daisy_cli::{DaisyCli, Commands};
use crate::{daisy_help::handlerequests::handle_request};
use crate::{daisy_conf::load_config};

// use hyper::{Request, Response, body::Incoming};
use hyper::service::service_fn;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;





#[tokio::main]
async fn main() {
    let mut config = load_config();
    let cli = DaisyCli::parse();

    match cli.entity_type {
        Commands::Start(args) => {
            if let Some(d_port) = args.port {
                config.server.port = d_port;
            }

            let address = format!("{}:{}", config.server.host, config.server.port);
            let listener = TcpListener::bind(&address).await.unwrap();

            println!("🌼 Daisy running on http://{}", address);

            loop {
                let (stream, _) = listener.accept().await.unwrap();
                let io = TokioIo::new(stream);

                tokio::spawn(async move {

                    if let Err(err) = http1::Builder::new()
                        .serve_connection( io , service_fn(handle_request))
                        .await
                    {
                        eprintln!("Error: {:?}", err);
                    }
                });
            }
        }

        _ => {
            println!("wrong command!!");
        }
    }
}