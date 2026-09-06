//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/mmc-davinci.h
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
//
// Board-specific MMC configuration
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_mmc_config {
// get_cd()/get_wp() may sleep
    pub module): *mut *mut int (get_cd)(int,
    pub module): *mut *mut int (get_ro)(int,
    pub on): *mut *mut void (set_power)(int module, bool,
// wires == 0 is equivalent to wires == 4 (4-bit parallel)
    pub wires: u8,
    pub max_freq: u32,
// any additional host capabilities: OR'd in to mmc->f_caps
    pub caps: u32,
// Number of sg segments
    pub nr_sg: u8,
}

extern "C" {
    pub fn davinci_setup_mmc(module: c_int, config: *mut davinci_mmc_config);
}
