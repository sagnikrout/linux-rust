//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/omap-gpmc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// OMAP GPMC (General Purpose Memory Controller) defines
//

pub const GPMC_CONFIG_WP: c_uint = 0x00000005;
// IRQ numbers in GPMC IRQ domain for legacy boot use
pub const GPMC_IRQ_FIFOEVENTENABLE: c_int = 0;
pub const GPMC_IRQ_COUNT_EVENT: c_int = 1;
//
// gpmc_nand_ops - Interface between NAND and GPMC
// @nand_write_buffer_empty: get the NAND write buffer empty status.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmc_nand_ops {
    pub (*nand_writebuffer_empty)(void): *mut bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmc_onenand_info {
    pub sync_read: bool,
    pub sync_write: bool,
    pub burst_len: c_int,
}

//
// gpmc_omap_onenand_set_timings - set optimized sync timings.
// @cs:      Chip Select Region
// @freq:    Chip frequency
// @latency: Burst latency cycle count
// @info:    Structure describing parameters used
//
// Sets optimized timings for the @cs region based on @freq and @latency.
// Updates the @info structure based on the GPMC settings.
//

extern "C" {
    pub fn gpmc_cs_write_reg(cs: c_int, idx: c_int, val: u32);
}
extern "C" {
    pub fn gpmc_calc_divider(sync_clk: c_uint) -> c_int;
}
extern "C" {
    pub fn gpmc_cs_program_settings(cs: c_int, p: *mut gpmc_settings) -> c_int;
}
extern "C" {
    pub fn gpmc_cs_request(cs: c_int, size: c_ulong, base: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn gpmc_cs_free(cs: c_int);
}
extern "C" {
    pub fn gpmc_configure(cmd: c_int, wval: c_int) -> c_int;
}
