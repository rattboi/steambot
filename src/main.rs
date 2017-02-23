extern crate irc;

use std::default::Default;
use irc::client::prelude::*;

fn main() {
  let cfg = Config {
    nickname: Some(format!("irc-rs")),
    server: Some(format!("irc.cat.pdx.edu")),
    channels: Some(vec![format!("#steam")]),
    .. Default::default()
  };
  let server = IrcServer::from_config(cfg).unwrap();
  server.identify().unwrap();
  for message in server.iter() {
    println!("{}", message.unwrap());
  }
}
