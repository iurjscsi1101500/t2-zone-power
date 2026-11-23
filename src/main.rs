// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 André Eikmeyer <andre.eikmeyer@kait2en.org>
// Copyright (C) 2026 Atharva Tiwari <atharvatiwarilinuxdev@gmail.com>

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
