//! Automatically rewritten from C Header to Rust Module
//! Source: net/netlabel/netlabel_user.h
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
// NetLabel NETLINK Interface
//
// This file defines the NETLINK interface for the NetLabel system.  The
// NetLabel system manages static and dynamic label mappings for network
// protocols such as CIPSO and RIPSO.
//
// Author: Paul Moore <paul@paul-moore.com>
//
// (c) Copyright Hewlett-Packard Development Company, L.P., 2006
//

// NetLabel NETLINK helper functions
//
// netlbl_netlink_auditinfo - Fetch the audit information from a NETLINK msg
// @audit_info: NetLabel audit information
//
// NetLabel NETLINK I/O functions
extern "C" {
    pub fn netlbl_netlink_init() -> c_int;
}
// NetLabel Audit Functions
