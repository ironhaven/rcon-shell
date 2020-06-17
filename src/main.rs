//! CLI to administer command to Source or Minecraft servers
//! Copyright (C) 2020 Nicholas Farley
//!
//! This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

use directories::ProjectDirs;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::error::Error;
use std::fs;
use std::io;
use structopt::StructOpt;
use tokio::runtime::Runtime;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "rcon-shell",
    about = "CLI to administer command to Source or Minecraft servers",
    author = "Copyright (C) 2020 Nicholas Farley"
)]
struct Opt {
    /// Domain name or ip address of server to connect to.
    #[structopt(short = "H", long)]
    host: String,
    /// Port number of server to connect to.
    #[structopt(short = "P", long, default_value = "25575")]
    port: u16,
    /// RCON password to server. Can be given in the cli.
    /// The password input will be hidden in the terminal.
    #[structopt(short, long)]
    password: Option<String>,
}

const BANNER: &'static str = concat!(
    "rcon-shell ",
    env!("CARGO_PKG_VERSION"),
    " Copyright (C) 2020 Nicholas Farley
This program comes with ABSOLUTELY NO WARRANTY
This is free software, and you are welcome to redistribute it under certain conditions
for more info go to https://www.gnu.org/licenses/gpl-3.0.en.html

Enter 'Q' to quit"
);

fn main() -> Result<(), Box<dyn Error>> {
    let mut rt = Runtime::new()?;
    let history_path = ProjectDirs::from("net", "ironhaven", "rcon-shell")
        .unwrap()
        .cache_dir()
        .join("history.txt");

    let mut rl = scopeguard::guard(Editor::<()>::new(), |rl| {
        rl.save_history(&history_path).unwrap();
    });
    if let Err(ReadlineError::Io(e)) = rl.load_history(&history_path) {
        if let io::ErrorKind::NotFound = e.kind() {
            fs::create_dir_all(&history_path.parent().unwrap())?;
            fs::File::create(&history_path)?;
        }
    }
    let opt = Opt::from_args();
    let mut rcon = rt.block_on(rcon::Connection::connect(
        (opt.host.as_str(), opt.port),
        &opt.password
            .unwrap_or_else(|| rpassword::read_password_from_tty(Some("rcon password: ")).unwrap()),
    ))?;
    println!("{}", BANNER);
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(quit) if quit == "Q" => {
                break;
            }
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let resp = match rt.block_on(rcon.cmd(&line)) {
                    Ok(line) => line,
                    Err(e) => {
                        println!("backing up");
                        return Err(Box::new(e));
                    }
                };
                if !resp.is_empty() {
                    println!("{}", resp);
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                return Err(Box::new(err));
            }
        }
    }
    Ok(())
}
