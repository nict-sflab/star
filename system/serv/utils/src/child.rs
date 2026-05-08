use std::{io, vec};
use crate::io as utilsio;

pub trait Commands {
    fn handle(&mut self, operation: u8, payload :&[u8]) -> (u8, vec::Vec<u8>);
}

pub fn run_stdio<C: Commands>(commands: &mut C) -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    loop {
        let Some(request) = utilsio::read_request(&mut stdin)? else {
            return Ok(());
        };
        let (status, response) = commands.handle(request.op, &request.payload);
        utilsio::write_response(&mut stdout, status, &response)?;
    }
}
