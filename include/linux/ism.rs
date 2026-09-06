//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ism.h
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
// Internal Shared Memory
//
// Definitions for the ISM module
//
// Copyright IBM Corp. 2022
//

// Unless we gain unexpected popularity, this limit should hold for a while
pub const MAX_CLIENTS: c_int = 8;
pub const ISM_NR_DMBS: c_int = 1920;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ism_dev {
    pub /: *mut *mut spinlock_t lock; / protects the ism device,
    pub /: *mut *mut spinlock_t cmd_lock; / serializes cmds,
    pub list: list_head,
    pub dibs: *mut dibs_dev,
    pub pdev: *mut pci_dev,
    pub sba: *mut ism_sba,
    pub sba_dma_addr: dma_addr_t,
    pub ISM_NR_DMBS): DECLARE_BITMAP(sba_bitmap,,
    pub priv: [*mut c_void; MAX_CLIENTS],
    pub ieq: *mut ism_eq,
    pub ieq_dma_addr: dma_addr_t,
    pub ieq_idx: c_int,
    pub subs: [*mut ism_client; MAX_CLIENTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ism_event {
    pub type: u32,
    pub code: u32,
    pub tok: u64,
    pub time: u64,
    pub info: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ism_client {
    pub name: *const c_char,
    pub event): *mut *mut *mut void (handle_event)(struct ism_dev dev, struct ism_event,
// Private area - don't touch!
    pub id: u8,
}

extern "C" {
    pub fn ism_register_client(client: *mut ism_client) -> c_int;
}
extern "C" {
    pub fn ism_unregister_client(client: *mut ism_client) -> c_int;
}
