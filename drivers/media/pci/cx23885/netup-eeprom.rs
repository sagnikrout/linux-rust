//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx23885/netup-eeprom.h
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
// netup-eeprom.h
//
// 24LC02 EEPROM driver in conjunction with NetUP Dual DVB-S2 CI card
//
// Copyright (C) 2009 NetUP Inc.
// Copyright (C) 2009 Abylay Ospan <aospan@netup.ru>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netup_port_info {
    pub /: *mut *mut u8 mac[6];/ card MAC address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netup_card_info {
    pub /: *mut *mut netup_port_info port[2];/ ports - 1,2,
    pub /: *mut *mut u8 rev;/ card revision,
}

extern "C" {
    pub fn netup_eeprom_read(i2c_adap: *mut i2c_adapter, addr: u8) -> c_int;
}
