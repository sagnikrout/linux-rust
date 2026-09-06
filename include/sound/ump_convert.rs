//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ump_convert.h
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

// context for converting from legacy control messages to UMP packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ump_cvt_to_ump_bank {
    pub rpn_set: bool,
    pub nrpn_set: bool,
    pub bank_set: bool,
    pub cc_rpn_lsb: unsigned char cc_rpn_msb,,
    pub cc_nrpn_lsb: unsigned char cc_nrpn_msb,,
    pub cc_data_lsb: unsigned char cc_data_msb,,
    pub cc_bank_lsb: unsigned char cc_bank_msb,,
    pub cc_data_lsb_set: bool cc_data_msb_set,,
}

// context for converting from MIDI1 byte stream to UMP packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ump_cvt_to_ump {
// MIDI1 intermediate buffer
    pub /: *mut *mut unsigned char buf[6]; / up to 6 bytes for SysEx,
    pub len: c_int,
    pub cmd_bytes: c_int,
// UMP output packet
    pub ump: [u32; 4],
    pub ump_bytes: c_int,
// various status
    pub in_sysex: c_uint,
    pub /: *mut *mut ump_cvt_to_ump_bank bank[16]; / per channel,
}

// reset the converter context, called at each open to ump
