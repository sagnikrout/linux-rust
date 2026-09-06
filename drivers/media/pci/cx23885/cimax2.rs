//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx23885/cimax2.h
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
// cimax2.h
//
// CIMax(R) SP2 driver in conjunction with NetUp Dual DVB-S2 CI card
//
// Copyright (C) 2009 NetUP Inc.
// Copyright (C) 2009 Igor M. Liplianin <liplianin@netup.ru>
// Copyright (C) 2009 Abylay Ospan <aospan@netup.ru>
//

extern "C" {
    pub fn netup_ci_slot_reset(en50221: *mut dvb_ca_en50221, slot: c_int) -> c_int;
}
extern "C" {
    pub fn netup_ci_slot_shutdown(en50221: *mut dvb_ca_en50221, slot: c_int) -> c_int;
}
extern "C" {
    pub fn netup_ci_slot_ts_ctl(en50221: *mut dvb_ca_en50221, slot: c_int) -> c_int;
}
extern "C" {
    pub fn netup_ci_slot_status(dev: *mut cx23885_dev, pci_status: u32) -> c_int;
}
extern "C" {
    pub fn netup_ci_init(port: *mut cx23885_tsport) -> c_int;
}
extern "C" {
    pub fn netup_ci_exit(port: *mut cx23885_tsport);
}
