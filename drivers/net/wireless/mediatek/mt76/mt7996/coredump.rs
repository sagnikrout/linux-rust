//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7996/coredump.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2023 MediaTek Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_coredump {
    pub magic: [c_char; 16],
    pub len: u32,
    pub guid: guid_t,
// time-of-day stamp
    pub tv_sec: u64,
// time-of-day stamp, nano-seconds
    pub tv_nsec: u64,
// kernel version
    pub kernel: [c_char; 64],
// firmware version
    pub fw_ver: [c_char; ETHTOOL_FWVERS_LEN],
    pub device_id: u32,
// exception state
    pub fw_state: [c_char; 12],
// program counters
    pub pc_current: [c_char; 16],
    pub pc_stack: [u32; 17],
// link registers
    pub lr_stack: [u32; 16],
// memory content
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_coredump_mem {
    pub len: u32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mem_hdr {
    pub start: u32,
    pub len: u32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mem_region {
    pub start: u32,
    pub len: usize,
    pub name: *const c_char,
}

extern "C" {
    pub fn mt7996_coredump_submit(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_coredump_register(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_coredump_unregister(dev: *mut mt7996_dev);
}

