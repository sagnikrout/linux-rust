//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/purelifi/plfxlc/chip.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 pureLiFi
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum unit_type {
    STA = 0,
    AP = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plfxlc_chip {
    pub usb: plfxlc_usb,
    pub /: *mut *mut mutex mutex; / lock to protect chip data,
    pub unit_type: unit_type,
    pub link_led: u16,
    pub beacon_set: u8,
    pub beacon_interval: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plfxlc_mc_hash {
    pub low: u32,
    pub high: u32,
}

extern "C" {
    pub fn plfxlc_chip_release(chip: *mut plfxlc_chip);
}
extern "C" {
    pub fn plfxlc_chip_disable_rxtx(chip: *mut plfxlc_chip);
}
extern "C" {
    pub fn plfxlc_chip_init_hw(chip: *mut plfxlc_chip) -> c_int;
}
extern "C" {
    pub fn plfxlc_chip_enable_rxtx(chip: *mut plfxlc_chip) -> c_int;
}
extern "C" {
    pub fn plfxlc_chip_set_rate(chip: *mut plfxlc_chip, rate: u8) -> c_int;
}
extern "C" {
    pub fn plfxlc_chip_switch_radio(chip: *mut plfxlc_chip, value: u16) -> c_int;
}
// usb)
extern "C" {
    pub fn container_of(_arg: usb, plfxlc_chip: struct, _arg: usb) -> return;
}
