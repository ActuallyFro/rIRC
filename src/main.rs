use std::io::{self, Write, BufRead, BufReader};
use std::net::TcpStream;

fn main() {
  println!("Rust IRC");
  println!("---------");

  // Get the server IP and port
  let mut input = String::new();
  println!("Enter server IP:PORT (e.g., 127.0.0.1:6667): ");
  io::stdin().read_line(&mut input).unwrap();
  let server_addr = input.trim();
  let mut stream = match TcpStream::connect(server_addr) {
    Ok(stream) => stream,
    Err(_) => {
      println!("Failed to connect to server at {}", server_addr);
      return;
    }
  };
  println!("Connected to {}", server_addr);
  println!("[server] Welcome to the IRC server!");

  // Read incoming server messages in a separate thread
  let mut reader = BufReader::new(stream.try_clone().unwrap());
  std::thread::spawn(move || {
    loop {
      let mut server_msg = String::new();
      if reader.read_line(&mut server_msg).unwrap() == 0 {
        break;
      }
      let trimmed_msg = server_msg.trim();
      if let Some(pos) = trimmed_msg.find(" :") {
        let sender = &trimmed_msg[1..trimmed_msg.find(' ').unwrap_or(0)];
        let message = &trimmed_msg[pos + 2..];
        if message.contains("::") {
          println!("[{}] {}", sender, message.replace("::", ":"));
        } else {
          println!("[{}] {}", sender, message);
        }
      } else {
        let sender = &trimmed_msg[1..trimmed_msg.find(' ').unwrap_or(0)];
        println!("[{}] {}", sender, trimmed_msg);
      }
    }
  });

  // Command-line interface for the client
  let mut nickname = String::new();
  let mut channel = String::new();

  loop {
    // Ask user for a command
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let command = input.trim();

    // Handle commands
    if command.starts_with("/nick ") {
      let new_nick = &command[6..];
      if !new_nick.is_empty() {
        nickname = new_nick.to_string();
        writeln!(stream, "NICK {}", nickname).unwrap();
        println!("Your nickname is now: {}", nickname);
      } else {
        println!("Invalid nickname.");
      }
    } else if command.starts_with("/join ") {
      if nickname.is_empty() {
        println!("You must set a nickname first using /nick.");
        continue;
      }
      let new_channel = &command[6..];
      if !new_channel.is_empty() {
        channel = new_channel.to_string();
        writeln!(stream, "JOIN {}", channel).unwrap();
        println!("Joined channel: {}", channel);
      } else {
        println!("Invalid channel.");
      }
    } else if command.starts_with("/users") {
      writeln!(stream, "NAMES {}", channel).unwrap();
    } else if command.starts_with("/help") {
      println!("[server] Supported commands: /nick <name>, /join <channel>, /users, /help, /quit");
    } else if command.starts_with("/") && !command.starts_with("/quit") {
    } else if command.starts_with("/") && !command.starts_with("/quit") {
      println!("Unknown command: {}", command);
    } else if command == "/quit" {
      writeln!(stream, "QUIT").unwrap();
      println!("Disconnecting...");
      break;
    } else if !channel.is_empty() {
      if !channel.starts_with('#') {
      println!("Invalid channel name. Channel names must start with '#'.");
      } else {
      writeln!(stream, "PRIVMSG {} :{}", channel, command).unwrap();
      }
    } else {
      println!("You must join a channel before sending messages.");
    }
  }
}