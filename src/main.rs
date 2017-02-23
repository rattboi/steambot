extern crate irc;

use irc::client::prelude::*;

struct SteamCommand<'a> {
  server: &'a IrcServer,
  target: &'a String,
  args: Vec<&'a str>
}

impl<'a> SteamCommand<'a> {
  fn new(server: &'a IrcServer, target: &'a String, msg: &'a String) -> Result<SteamCommand<'a>, &'static str> {
    let args:Vec<&str> = msg.split_at(1).1.split(' ').collect();
    let command = args[0];

    match command.to_lowercase().as_ref() {
      "reg" | 
      "price" | 
      "now" |
      "game" => { 
        Ok(SteamCommand {
          server: server,
          target: target,
          args: args
        })
      },
      _ => Err("No such command")
    }
  }
}

fn handle_command(command: SteamCommand) {
  for argument in command.args {
    command.server.send_privmsg(command.target, argument).unwrap();
  }
}

fn handle_privmsg(server: &IrcServer, target: &String, msg: &String) {
  let command = SteamCommand::new(server, target, msg);
  match command {
    Ok(command) => handle_command(command),
    Err(err_msg) => print!("{}", err_msg)
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
