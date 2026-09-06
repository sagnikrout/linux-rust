//! Automatically rewritten from C Header to Rust Module
//! Source: net/netlabel/netlabel_addrlist.h
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
// NetLabel Network Address Lists
//
// This file contains network address list functions used to manage ordered
// lists of network addresses for use by the NetLabel subsystem.  The NetLabel
// system manages static and dynamic label mappings for network protocols such
// as CIPSO and RIPSO.
//
// Author: Paul Moore <paul@paul-moore.com>
//
// (c) Copyright Hewlett-Packard Development Company, L.P., 2008
//

//
// struct netlbl_af4list - NetLabel IPv4 address list
// @addr: IPv4 address
// @mask: IPv4 address mask
// @valid: valid flag
// @list: list structure, used internally
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_af4list {
    pub addr: __be32,
    pub mask: __be32,
    pub valid: u32,
    pub list: list_head,
}

//
// struct netlbl_af6list - NetLabel IPv6 address list
// @addr: IPv6 address
// @mask: IPv6 address mask
// @valid: valid flag
// @list: list structure, used internally
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_af6list {
    pub addr: in6_addr,
    pub mask: in6_addr,
    pub valid: u32,
    pub list: list_head,
}

extern "C" {
    pub fn netlbl_af4list_remove_entry(entry: *mut netlbl_af4list);
}

extern "C" {
    pub fn netlbl_af6list_remove_entry(entry: *mut netlbl_af6list);
}

