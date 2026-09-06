//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/mantis/mantis_ioc.h
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
pub const GPIF_A00: c_uint = 0x00;
pub const GPIF_A01: c_uint = 0x01;
pub const GPIF_A02: c_uint = 0x02;
pub const GPIF_A03: c_uint = 0x03;
pub const GPIF_A04: c_uint = 0x04;
pub const GPIF_A05: c_uint = 0x05;
pub const GPIF_A06: c_uint = 0x06;
pub const GPIF_A07: c_uint = 0x07;
pub const GPIF_A08: c_uint = 0x08;
pub const GPIF_A09: c_uint = 0x09;
pub const GPIF_A10: c_uint = 0x0a;
pub const GPIF_A11: c_uint = 0x0b;
pub const GPIF_A12: c_uint = 0x0c;
pub const GPIF_A13: c_uint = 0x0d;
pub const GPIF_A14: c_uint = 0x0e;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mantis_stream_control {
    STREAM_TO_HIF = 0,
    STREAM_TO_CAM
}

extern "C" {
    pub fn mantis_get_mac(mantis: *mut mantis_pci) -> c_int;
}
extern "C" {
    pub fn mantis_gpio_set_bits(mantis: *mut mantis_pci, bitpos: u32, value: u8);
}
extern "C" {
    pub fn mantis_stream_control(mantis: *mut mantis_pci, stream_ctl: mantis_stream_control) -> c_int;
}
