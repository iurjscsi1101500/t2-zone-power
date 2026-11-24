// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 André Eikmeyer <andre.eikmeyer@kait2en.org>
// Copyright (C) 2026 Atharva Tiwari <atharvatiwarilinuxdev@gmail.com>

use std::{
    fs::{File, OpenOptions},
    os::fd::AsRawFd,
};

use crate::error::{Result, ZonePowerError};

#[repr(C)]
struct Payload {
    key: u32,
    size: u32,
    buf: [u8; 32],
}

mod ioctls {
    nix::ioctl_readwrite!(read, b'S', 1, super::Payload);
    nix::ioctl_write_ptr!(write, b'S', 2, super::Payload);
}

pub struct Smc(File);

fn io(e: impl Into<std::io::Error>) -> ZonePowerError {
    ZonePowerError::Io { path: "/dev/smc".into(), source: e.into() }
}

fn payload(key: &str, value: u8) -> Payload {
    let key = u32::from_be_bytes(key.as_bytes().try_into().expect("SMC key must be 4 bytes"));
    let mut buf = [0; 32];
    buf[0] = value;
    Payload { key, size: 1, buf }
}

impl Smc {
    pub fn open() -> Result<Self> {
        OpenOptions::new().read(true).write(true).open("/dev/smc").map(Self).map_err(io)
    }

    pub fn read_u8(&self, key: &str) -> Result<u8> {
        let mut p = payload(key, 0);
        unsafe { ioctls::read(self.0.as_raw_fd(), &mut p) }.map_err(io)?;
        Ok(p.buf[0])
    }

    pub fn write_u8(&self, key: &str, value: u8) -> Result<()> {
        let p = payload(key, value);
        unsafe { ioctls::write(self.0.as_raw_fd(), &p) }.map_err(io)?;
        Ok(())
    }
}
