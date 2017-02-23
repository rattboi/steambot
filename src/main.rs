extern crate irc;

use irc::client::prelude::*;

fn handle_command(server: &IrcServer, target: &String, args: Vec<&str>) {
  for argument in args {
    server.send_privmsg(target, argument).unwrap();
  }
}

fn handle_privmsg(server: &IrcServer, target: &String, msg: &String) {
  let args:Vec<&str> = msg.split_at(1).1.split(' ').collect();
  let command = args[0];

  match command.to_lowercase().as_ref() {
    "reg" | 
    "price" | 
    "now" |
    "game" => { 
      handle_command(server, target, args);
    },
    _ => ()
  }
}

fn main() {
  let server = IrcServer::new("config.json").unwrap();
  server.identify().unwrap();
  for message in server.iter() {
    let message = message.unwrap();
    print!("{}", message);
    match message.command {
      Command::PRIVMSG(ref target, ref msg) => if msg.starts_with("!") {
        handle_privmsg(&server, target, msg)
      },
      _ => (),
    }
  }
}
