use keepass::{
    config::DatabaseVersion, db::NodeRef, error::DatabaseOpenError, Database, DatabaseKey,
};
use std::{fs::File, io::Read};

fn main() -> Result<(), DatabaseOpenError> {
    // Open KeePass database using a password (keyfile is also supported)
    let mut file = File::open("redhome.kdbx")?;
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
