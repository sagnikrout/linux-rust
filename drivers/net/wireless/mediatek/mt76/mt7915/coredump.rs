//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7915/coredump.h
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
// Copyright (C) 2022 MediaTek Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace {
    pub id: u32,
    pub timestamp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_coredump {
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
    pub last_msg_id: u32,
    pub eint_info_idx: u32,
    pub irq_info_idx: u32,
    pub sched_info_idx: u32,
// schedule info
    pub trace_sched: [c_char; 32],
    pub t: trace,
    pub pc: u32,
    pub sched: [}; 60],
// irq info
    pub trace_irq: [c_char; 32],
    pub irq: [trace; 60],
// task queue status
    pub task_qid: [c_char; 32],
    pub read: u32,
    pub write: u32,
    pub taskq: [}; 2],
// task stack info
    pub task_info: [c_char; 32],
    pub start: u32,
    pub end: u32,
    pub size: u32,
    pub taski: [}; 2],
// firmware context
    pub fw_context: [c_char; 24],
    pub idx: u32,
    pub handler: u32,
    pub context: },
// link registers calltrace
    pub call_stack: [u32; 16],
// memory content
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_coredump_mem {
    pub len: u32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mem_hdr {
    pub start: u32,
    pub len: u32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mem_region {
    pub start: u32,
    pub len: usize,
    pub name: *const c_char,
}

extern "C" {
    pub fn mt7915_coredump_submit(dev: *mut mt7915_dev) -> c_int;
}
extern "C" {
    pub fn mt7915_coredump_register(dev: *mut mt7915_dev) -> c_int;
}
extern "C" {
    pub fn mt7915_coredump_unregister(dev: *mut mt7915_dev);
}

