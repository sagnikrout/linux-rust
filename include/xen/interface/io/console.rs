//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/console.h
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


// SPDX-License-Identifier: MIT
//
// console.h
//
// Console I/O interface for Xen guest OSes.
//
// Copyright (c) 2005, Keir Fraser
//
pub type XENCONS_RING_IDX = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xencons_interface {
    pub in: [c_char; 1024],
    pub out: [c_char; 2048],
    pub in_prod: XENCONS_RING_IDX in_cons,,
    pub out_prod: XENCONS_RING_IDX out_cons,,
//
// Flag values signaling from backend to frontend whether the console is
// connected.  i.e. Whether it will be serviced and emptied.
//
// The flag starts as disconnected.
//
pub const XENCONSOLE_DISCONNECTED: c_int = 1;
//
// The flag is set to connected when the backend connects and the console
// will be serviced.
//
pub const XENCONSOLE_CONNECTED: c_int = 0;
    pub connection: u8,
}
