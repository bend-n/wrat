use comat::comat;
use std::{io::ErrorKind, process::ExitCode};

macro_rules! fail {
    () => {
        fail!("[$($x:u8),+ $(,)?] <outfile>")
    };
    ($usage:literal) => {{
        eprintln!(concat!("usage: wrat ", comat!($usage)));
        return ExitCode::FAILURE;
    }};
}
fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let first = args.next();
    let Some(Some(first)) = first.map(|x| x.strip_prefix('[').map(ToOwned::to_owned)) else {
        fail!()
    };
    let Ok(n) = first.trim_end_matches(',').parse::<u8>() else {
        fail!("[{red}u8{reset}]");
    };

    let mut dat = Vec::with_capacity(64);
    dat.push(n);
    loop {
        let Some(x) = args.next() else {
            fail!("[{red}u8{reset}]")
        };
        let Ok(n) = x.trim_end_matches(',').trim_end_matches(']').parse::<u8>() else {
            fail!("[{red}u8{reset}]");
        };
        dat.push(n);
        match x.strip_suffix(']') {
            Some(_) => break,
            None => continue,
        }
    }
    let Some(out) = args.next() else {
        fail!("[..] <{red}filename{reset}>")
    };
    match std::fs::write(out, dat).map_err(|e| e.kind()) {
        Ok(()) => {}
        Err(ErrorKind::PermissionDenied) => {
            fail!("[..] <{red}accessible filename{reset}>")
        }
        Err(ErrorKind::NotFound) => {
            fail!("[..] <{red}valid filename{reset}>")
        }
        Err(_) => {
            fail!("[..] <{red}writable file{reset}>")
        }
    }
    ExitCode::SUCCESS
}
