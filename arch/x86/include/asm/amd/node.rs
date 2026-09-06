//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/amd/node.h
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
// AMD Node helper functions and common defines
//
// Copyright (c) 2024, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Yazen Ghannam <Yazen.Ghannam@amd.com>
//
// Note:
// Items in this file may only be used in a single place.
// However, it's prudent to keep all AMD Node functionality
// in a unified place rather than spreading throughout the
// kernel.
//

pub const MAX_AMD_NUM_NODES: c_int = 8;
pub const AMD_NODE0_PCI_SLOT: c_uint = 0x18;
extern "C" {
    pub fn topology_amd_nodes_per_pkg(topology_max_packages(: *mut *mut )) -> return;
}

extern "C" {
    pub fn amd_smn_read(node: u16, address: u32, value: *mut u32) -> int __must_check;
}
extern "C" {
    pub fn amd_smn_write(node: u16, address: u32, value: u32) -> int __must_check;
}
// Should only be used by the HSMP driver.
extern "C" {
    pub fn amd_smn_hsmp_rdwr(node: u16, address: u32, value: *mut u32, write: bool) -> int __must_check;
}

// helper for use with read_poll_timeout
