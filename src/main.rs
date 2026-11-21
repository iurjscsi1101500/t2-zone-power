mod actuators;
mod daemon;
mod error;
mod plimit;
mod sysfs;
mod smc;

fn main() -> glib::ExitCode {
    let debug = std::env::args().any(|arg| arg == "--debug");
    if let Err(error) = daemon::run(debug) {
        eprintln!("{error}");
        return glib::ExitCode::from(1);
    }

    glib::ExitCode::SUCCESS
}
