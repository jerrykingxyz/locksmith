/*use keepass::{
    config::DatabaseVersion, db::NodeRef, error::DatabaseOpenError, Database, DatabaseKey,
};
use std::{fs::File, io::Read};

fn main() -> Result<(), DatabaseOpenError> {
    // Open KeePass database using a password (keyfile is also supported)
    let mut file = File::open("aaa.kdbx")?;
    let key = DatabaseKey::new().with_password("**");
    let db = Database::open(&mut file, key)?;
    println!("aaa {:?}", db);

    // Iterate over all `Group`s and `Entry`s
    for node in &db.root {
        match node {
            NodeRef::Group(g) => {
                println!("Saw group '{0}'", g.name);
            }
            NodeRef::Entry(e) => {
                let title = e.get_title().unwrap_or("(no title)");
                let user = e.get_username().unwrap_or("(no username)");
                let pass = e.get_password().unwrap_or("(no password)");
                println!("Entry '{0}': '{1}' : '{2}'", title, user, pass);
            }
        }
    }

    Ok(())
}
 */

mod client;

// cli -> cmd -> backend -> consumer
// server -> api -> backend -> consumer
// provate --> kdbx or other file --> trait

fn main() -> Result<(), String> {
    let mut cli = client::cli::Cli::default();
    cli.listen();
    Ok(())
    /*    loop {
        std::io::stdout().flush().map_err(|e| e.to_string())?;
        write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
        std::io::stdout().flush().map_err(|e| e.to_string())?;
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .map_err(|e| e.to_string())?;
        let line = buffer.trim();
        if line.is_empty() {
            continue;
        }

        cli.try_match_args(line);
    }*/
}
