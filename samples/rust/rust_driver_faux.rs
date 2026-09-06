
// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------

// SPDX-License-Identifier: GPL-2.0-only

//! Rust faux device sample.

use kernel::{
    faux,
    prelude::*,
    Module, //
};

module! {
    type: SampleModule,
    name: "rust_faux_driver",
    authors: ["Lyude Paul"],
    description: "Rust faux device sample",
    license: "GPL",
}

struct SampleModule {
    _reg: faux::Registration,
}

impl Module for SampleModule {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Initialising Rust Faux Device Sample\n");

        let reg = faux::Registration::new(c"rust-faux-sample-device", None)?;
        let fdev = reg.as_ref();

        dev_info!(fdev, "Hello from faux device!\n");

        Ok(Self { _reg: reg })
    }
}
