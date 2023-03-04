use inotify::{Inotify, WatchMask};
use std::{
    path::Path,
};

fn main() {
    //let path: String  = get_path_arg();

    let mut inotify = Inotify::init().expect("Error while initializing inotify instance");
    println!("Test");

    // Watch for modify and close events.
    inotify
        .add_watch(
            "/data/dump.rdb",
            WatchMask::ALL_EVENTS
        )
        .expect("Failed to add file watch");

    // Read events that were added with `add_watch` above.
    let mut buffer = [0; 1024];
    let events = inotify
        .read_events_blocking(&mut buffer)
        .expect("Error while reading events");

    for event in events {
        println!("{event:?}");
    }
}

fn get_path_arg() -> String {
    let args = std::env::args();

    if args.len() != 2 {
        panic!("Arg error: expected 1 argument, found {}", args.len()-1);
    }

    let arg = &args.collect::<Vec<String>>()[1];

    if !Path::new(&arg).exists() {
        panic!("Path doesnt exists");
    }

    arg.clone()
}
