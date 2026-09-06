//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stb6100.h
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

pub const STB6100_LD: c_uint = 0x00;

pub const STB6100_VCO: c_uint = 0x01;

pub const STB6100_VCO_OSCH_SHIFT: c_int = 7;

pub const STB6100_VCO_OCK_SHIFT: c_int = 5;

pub const STB6100_VCO_ODIV_SHIFT: c_int = 4;

pub const STB6100_NI: c_uint = 0x02;
pub const STB6100_NF_LSB: c_uint = 0x03;
pub const STB6100_K: c_uint = 0x04;

pub const STB6100_K_PSD2_SHIFT: c_int = 2;

pub const STB6100_G: c_uint = 0x05;

pub const STB6100_F: c_uint = 0x06;

pub const STB6100_DLB: c_uint = 0x07;
pub const STB6100_TEST1: c_uint = 0x08;
pub const STB6100_FCCK: c_uint = 0x09;

pub const STB6100_LPEN: c_uint = 0x0a;

pub const STB6100_TEST3: c_uint = 0x0b;
pub const STB6100_NUMREGS: c_uint = 0x0c;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb6100_config {
    pub tuner_address: u8,
    pub refclock: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb6100_state {
    pub i2c: *mut i2c_adapter,
    pub config: *const stb6100_config,
    pub ops: dvb_tuner_ops,
    pub frontend: *mut dvb_frontend,
    pub frequency: u32,
    pub srate: u32,
    pub bandwidth: u32,
    pub reference: u32,
}

