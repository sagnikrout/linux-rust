//! Automatically rewritten from C to Rust
//! Source: tools/testing/nvdimm/config_check.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

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


// SPDX-License-Identifier: GPL-2.0

#[no_mangle]
pub unsafe extern "C" fn check() {
    void check(void)
    {
//
// These kconfig symbols must be set to "m" for nfit_test to
// load and operate.
//
    BUILD_BUG_ON(!IS_MODULE(CONFIG_LIBNVDIMM));
    BUILD_BUG_ON(!IS_MODULE(CONFIG_BLK_DEV_PMEM));
    BUILD_BUG_ON(!IS_MODULE(CONFIG_ND_BTT));
    BUILD_BUG_ON(!IS_MODULE(CONFIG_ND_PFN));
    if (IS_ENABLED(CONFIG_ACPI_NFIT))
    BUILD_BUG_ON(!IS_MODULE(CONFIG_ACPI_NFIT));
    BUILD_BUG_ON(!IS_MODULE(CONFIG_DEV_DAX));
    BUILD_BUG_ON(!IS_MODULE(CONFIG_DEV_DAX_PMEM));
    }
