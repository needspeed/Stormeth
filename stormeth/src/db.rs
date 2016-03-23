use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;

pub struct Database
{
    file : ::std::io::Result<File>,
}

impl Database
{
    fn commit(&mut self, data : &str)
    {
        match self.file
        {
            Ok(ref mut file) => { write!(file, "{}", data); },
            Err(ref err) => { println!("An error occured: {}", err); }
        }
    }
}

const PATH : &'static str = "/var/log/stormeth.fakedb";

pub fn get_database() -> Database
{
   Database
   {
       file : OpenOptions::new().read(true).write(true).create(true).open(PATH)
   }
}

#[test]
fn test()
{
    use std::io::Read;

    println!("start test");
    let db = get_database();
    db.commit("test");
}
