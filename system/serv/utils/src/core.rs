use std::{io::{ BufReader}, process::{Child, ChildStdin}};

pub type Subprocess = (Child, ChildStdin, BufReader<ChildStdin>);

pub const STATUS_OK : u8 = 0;