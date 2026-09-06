//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iscsi_ibft.h
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
// Copyright 2007 Red Hat, Inc.
// by Peter Jones <pjones@redhat.com>
// Copyright 2007 IBM, Inc.
// by Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Copyright 2008
// by Konrad Rzeszutek <ketuzsezr@darnok.org>
//
// This code exposes the iSCSI Boot Format Table to userland via sysfs.
//

//
// Physical location of iSCSI Boot Format Table.
// If the value is 0 there is no iBFT on the machine.
//

//
// Routine used to find and reserve the iSCSI Boot Format Table. The
// physical address is set in the ibft_phys_addr variable.
//
extern "C" {
    pub fn reserve_ibft_region();
}
//
// Physical bounds to search for the iSCSI Boot Format Table.
//
pub const IBFT_START: c_uint = 0x80000 /* 512kB */;
pub const IBFT_END: c_uint = 0x100000 /* 1MB */;

