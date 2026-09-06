//! Automatically rewritten from C Header to Rust Module
//! Source: net/dsa/trace.h
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
// Copyright 2022-2023 NXP
//

// Enough to fit "bridge %s num %d" where num has 3 digits

extern "C" {
    pub fn dsa_db_print(db: *const dsa_db, buf[DSA_DB_BUFSIZ]: c_char);
}
// Add unicast/multicast address to hardware, either on user ports
// (where no refcounting is kept), or on shared ports when the entry
// is first seen and its refcount is 1.
//
// Delete unicast/multicast address from hardware, either on user ports or
// when the refcount on shared ports reaches 0
//
// Bump the refcount of an existing unicast/multicast address on shared ports
// Drop the refcount of a multicast address that we still keep on
// shared ports
//
// Attempt to delete a unicast/multicast address on shared ports for which
// the delete operation was called more times than the addition
//

// We don't want to use include/trace/events

// This part must be outside protection
