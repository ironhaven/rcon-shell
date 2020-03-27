use directories::ProjectDirs;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::error::Error;
use std::fs;
use structopt::StructOpt;
use tokio::runtime::Runtime;

#[derive(StructOpt, Debug)]
#[structopt(name = "rcon-shell")]
struct Opt {
    /// Domain name or ip address of server to connect to
    #[structopt(short = "H", long)]
    host: String,
    /// Port number of server connect to
    #[structopt(short = "P", long, default_value = "25575")]
    port: u16,
    /// RCON password to server. Can be given in prompt
    /// The password input will be hidden in the prompt
    #[structopt(short, long)]
    password: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = Editor::<()>::new();
    let mut rt = Runtime::new()?;
    let history_path = ProjectDirs::from("net", "ironhaven", "rcon-shell")
        .unwrap()
        .cache_dir()
        .join("history.txt");
    if let Err(_) = rl.load_history(&history_path) {
        fs::create_dir_all(&history_path.parent().unwrap())?;
        fs::File::create(&history_path)?;
    }
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    let mut rcon = rt.block_on(rcon::Connection::connect(
        (opt.host.as_str(), opt.port),
        &opt.password
            .unwrap_or_else(|| rpassword::read_password_from_tty(Some("rcon password: ")).unwrap()),
    ))?;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("{}", line);
                println!("{}", rt.block_on(rcon.cmd(&line))?);
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
    rl.save_history(&history_path).unwrap();
    Ok(())
}
