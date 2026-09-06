//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/fsl/qe/tsa.h
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
// TSA management
//
// Copyright 2022 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>
//

extern "C" {
    pub fn tsa_serial_put(tsa_serial: *mut tsa_serial);
}
// Connect and disconnect the TSA serial
extern "C" {
    pub fn tsa_serial_connect(tsa_serial: *mut tsa_serial) -> c_int;
}
extern "C" {
    pub fn tsa_serial_disconnect(tsa_serial: *mut tsa_serial) -> c_int;
}
// Cell information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsa_serial_info {
    pub rx_fs_rate: c_ulong,
    pub rx_bit_rate: c_ulong,
    pub nb_rx_ts: u8,
    pub tx_fs_rate: c_ulong,
    pub tx_bit_rate: c_ulong,
    pub nb_tx_ts: u8,
}

// Get information
extern "C" {
    pub fn tsa_serial_get_info(tsa_serial: *mut tsa_serial, info: *mut tsa_serial_info) -> c_int;
}
// Get serial number
extern "C" {
    pub fn tsa_serial_get_num(tsa_serial: *mut tsa_serial) -> c_int;
}
