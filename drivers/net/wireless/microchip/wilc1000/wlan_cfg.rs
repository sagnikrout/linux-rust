//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/microchip/wilc1000/wlan_cfg.h
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
// Copyright (c) 2012 - 2018 Microchip Technology Inc., and its subsidiaries.
// All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_cfg_byte {
    pub id: u16,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_cfg_hword {
    pub id: u16,
    pub val: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_cfg_word {
    pub id: u16,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_cfg_str {
    pub id: u16,
    pub len: u16,
    pub str: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_cfg_str_vals {
    pub mac_address: [u8; 8],
    pub firmware_version: [u8; 130],
    pub assoc_rsp: [u8; WILC_MAX_ASSOC_RESP_FRAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_cfg {
    pub b: *mut wilc_cfg_byte,
    pub hw: *mut wilc_cfg_hword,
    pub w: *mut wilc_cfg_word,
    pub s: *mut wilc_cfg_str,
    pub str_vals: *mut wilc_cfg_str_vals,
}

extern "C" {
    pub fn wilc_wlan_cfg_set_wid(frame: *mut u8, offset: u32, id: u16, buf: *mut u8, size: c_int) -> c_int;
}
extern "C" {
    pub fn wilc_wlan_cfg_get_wid(frame: *mut u8, offset: u32, id: u16) -> c_int;
}
extern "C" {
    pub fn wilc_wlan_cfg_init(wl: *mut wilc) -> c_int;
}
extern "C" {
    pub fn wilc_wlan_cfg_deinit(wl: *mut wilc);
}
