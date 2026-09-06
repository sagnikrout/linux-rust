//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_eql.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Equalizer Load-balancer for serial network interfaces.
//
// (c) Copyright 1995 Simon "Guru Aleph-Null" Janes
// NCM: Network and Communications Management, Inc.
//
// This software may be used and distributed according to the terms
// of the GNU General Public License, incorporated herein by reference.
//
// The author may be reached as simon@ncm.com, or C/O
// NCM
// Attn: Simon Janes
// 6803 Whittier Ave
// McLean VA 22101
// Phone: 1-703-847-0040 ext 103
//
pub const EQL_DEFAULT_SLAVE_PRIORITY: c_int = 28800;
pub const EQL_DEFAULT_MAX_SLAVES: c_int = 4;
pub const EQL_DEFAULT_MTU: c_int = 576;

