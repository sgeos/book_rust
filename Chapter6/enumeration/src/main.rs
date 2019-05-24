use std::process;

enum Command {
  Exit,
  Move { x: i32, y: i32 },
  Message(String),
  SetColor(i32, i32, i32),
}

impl Command {
  fn execute(&self) {
    match self {
      Command::Exit => {
        println!("Exiting program.");
        process::exit(0);
      },
      Command::Move {x, y} => println!("Move to (x = {}, y = {}).", x, y),
      Command::Message(m) => println!("{}", m),
      Command::SetColor(r, g, b) => println!("Change color to (red = {}, green = {}, blue = {}).", r, g, b),
      //_ => panic!(),
    }
  }
}

fn main() {
  let command_list = [
    Some(Command::SetColor(255, 255, 255)),
    Some(Command::Message(String::from("Moving north."))),
    Some(Command::Move {x: 4, y: 0, }),
    Some(Command::SetColor(0, 0, 255)),
    None,
    Some(Command::Message(String::from("Moving west."))),
    Some(Command::Move {x: 0, y: -6, }),
    Some(Command::SetColor(255, 0, 0)),
    Some(Command::Exit),
    Some(Command::Message(String::from("Return to origin."))),
    None,
    None,
    Some(Command::Move {x: -4, y: 6, }),
    Some(Command::SetColor(0, 0, 0)),
    None,
  ];
  let verbose = false;
  for command in command_list.iter() {
    // if let multiple condition example
    if let (Some(Command::Exit), true) = (command, verbose) {
      println!("Ignoring exit command.");
    } else if let (Some(Command::Exit), false) = (command, verbose) {
      (); // only structured this way for example purposes
    // else if let example
    } else if let Some(command) = command {
      command.execute();
    // combine if let with else if (no let) example
    } else if verbose {
      println!("Missing command.");
    // combine if let with else example
    } else {
      (); // only structured this way for example purposes
    }
  };
}

