use std::time::SystemTime;

use log::{error, info};

mod nozomi;
mod pipes;

use nozomi::lua::NozomiLua;

static LUA_STR: &str = include_str!("../test.lua");

fn setup_logging() {
    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                // This will color the log level only, not the whole line. Just a touch.
                record.level(),
                record.target(),
                message
            ))
        })
        .apply()
        .unwrap();
}

#[poggers_derive::create_entry(no_console)]
fn entry() -> Result<(), Box<dyn std::error::Error>> {
    // We manually allocate a console because poggers is stupid.
    unsafe {
        poggers::exports::AllocConsole();
    }

    setup_logging();

    info!("DLL Injected");

    let lua = NozomiLua::new()?;

    match lua.load(LUA_STR) {
        Ok(_) => {
            info!("It executed.")
        }
        Err(err) => {
            error!("Got error: {err}");
        }
    };

    // TODO: Actually make namedpipes work :guh:
    // let cunt = ByteReaderPipeStream::connect("Nozomi")?;

    Ok(())
}
