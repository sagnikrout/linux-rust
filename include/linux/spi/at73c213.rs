//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/at73c213.h
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
// Board-specific data used to set up AT73c213 audio DAC driver.
//
// at73c213_board_info - how the external DAC is wired to the device.
//
// @ssc_id: SSC platform_driver id the DAC shall use to stream the audio.
// @dac_clk: the external clock used to provide master clock to the DAC.
// @shortname: a short discription for the DAC, seen by userspace tools.
//
// This struct contains the configuration of the hardware connection to the
// external DAC. The DAC needs a master clock and a I2S audio stream. It also
// provides a name which is used to identify it in userspace tools.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at73c213_board_info {
    pub ssc_id: c_int,
    pub dac_clk: *mut clk,
    pub shortname: [c_char; 32],
}
