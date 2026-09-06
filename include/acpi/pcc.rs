//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/pcc.h
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
// PCC (Platform Communications Channel) methods
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcc_mbox_chan {
    pub mchan: *mut mbox_chan,
    pub shmem_base_addr: u64,
    pub shmem: *mut void __iomem,
    pub shmem_size: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u16,
}

// Generic Communications Channel Shared Memory Region
pub const PCC_SIGNATURE: c_uint = 0x50434300;
// Generic Communications Channel Command Field

// Generic Communications Channel Status Field

// Initiator Responder Communications Channel Flags

pub const MAX_PCC_SUBSPACES: c_int = 256;

extern "C" {
    pub fn pcc_mbox_free_channel(chan: *mut pcc_mbox_chan);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

