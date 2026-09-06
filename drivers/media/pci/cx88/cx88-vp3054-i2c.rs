//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx88/cx88-vp3054-i2c.h
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
// cx88-vp3054-i2c.h  --  support for the secondary I2C bus of the
// DNTV Live! DVB-T Pro (VP-3054), wired as:
// GPIO[0] -> SCL, GPIO[1] -> SDA
//
// (c) 2005 Chris Pascoe <c.pascoe@itee.uq.edu.au>
//
// -----------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp3054_i2c_state {
    pub adap: i2c_adapter,
    pub algo: i2c_algo_bit_data,
    pub state: u32,
}

// -----------------------------------------------------------------------

extern "C" {
    pub fn vp3054_i2c_probe(dev: *mut cx8802_dev) -> c_int;
}
extern "C" {
    pub fn vp3054_i2c_remove(dev: *mut cx8802_dev);
}

