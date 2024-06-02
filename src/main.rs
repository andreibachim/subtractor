use clap::Parser;
use std::{
    io::{stdin, Read},
    path::Path,
    process::Command,
};

use notify::{event::CreateKind, Event, EventKind, RecursiveMode, Watcher};

fn main() -> notify::Result<()> {
    let args = Args::parse();
    println!("{:#?}", args);

    let mut watcher = notify::recommended_watcher(|res: Result<Event, notify::Error>| match res {
        Ok(event) => {
            if event.kind.eq(&EventKind::Create(CreateKind::File)) {
                let path = event
                    .paths
                    .first()
                    .expect("File should have at least on path");
                if !path
                    .extension()
                    .unwrap_or_default()
                    .eq_ignore_ascii_case("mkv")
                {
                    return;
                }

                let cmd = format!(
                    "ffmpeg -i {} -map 0:s:0 {}",
                    path.display(),
                    path.to_str().unwrap().replace("mkv", "srt")
                );
                let _ = Command::new("sh")
                    .args(vec!["-c", &cmd])
                    .output()
                    .expect("Could not run command");
                println!("Extraction completed!");
            }
        }
        Err(error) => eprintln!("Could not process event. {error}"),
    })?;

    watcher.watch(Path::new(&args.watch_dir), RecursiveMode::Recursive)?;
    stdin().read_to_string(&mut String::new())?;
    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    watch_dir: String,
}
