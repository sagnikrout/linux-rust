//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx23885/altera-ci.h
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
// altera-ci.c
//
// CI driver in conjunction with NetUp Dual DVB-T/C RF CI card
//
// Copyright (C) 2010 NetUP Inc.
// Copyright (C) 2010 Igor M. Liplianin <liplianin@netup.ru>
//
pub const ALT_DATA: c_uint = 0x000000ff;
pub const ALT_TDI: c_uint = 0x00008000;
pub const ALT_TDO: c_uint = 0x00004000;
pub const ALT_TCK: c_uint = 0x00002000;
pub const ALT_RDY: c_uint = 0x00001000;
pub const ALT_RD: c_uint = 0x00000800;
pub const ALT_WR: c_uint = 0x00000400;
pub const ALT_AD_RG: c_uint = 0x00000200;
pub const ALT_CS: c_uint = 0x00000100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altera_ci_config {
    pub /: *mut *mut *mut void dev;/ main dev, for example cx23885_dev,
    pub /: *mut *mut *mut void adapter;/ for CI to connect to,
    pub /: *mut *mut *mut dvb_demux demux;/ for hardware PID filter to connect to,
    pub rw): *mut *mut *mut int (fpga_rw) (void dev, int ad_rg, int val, int,
}

extern "C" {
    pub fn altera_ci_init(config: *mut altera_ci_config, ci_nr: c_int) -> c_int;
}
extern "C" {
    pub fn altera_ci_release(dev: *mut c_void, ci_nr: c_int);
}
extern "C" {
    pub fn altera_ci_irq(dev: *mut c_void) -> c_int;
}
extern "C" {
    pub fn altera_ci_tuner_reset(dev: *mut c_void, ci_nr: c_int) -> c_int;
}

