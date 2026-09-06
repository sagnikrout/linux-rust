//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/badblocks.h
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

pub const BB_MAX_LEN: c_int = 512;

// Bad block numbers are stored sorted in a single page.
// 64bits is used for each block or extent.
// 54 bits are sector number, 9 bits are extent size,
// 1 bit is an 'acknowledged' flag.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct badblocks {
    pub /: *mut *mut *mut device dev; / set by devm_init_badblocks,
    pub /: *mut *mut int count; / count of bad blocks,
    pub unacknowledged: *mut *mut int unacked_exist; / there probably are,
// bad blocks.  This is only cleared
// when a read discovers none
//
    pub size: *mut *mut int shift; / shift from sectors to block,
// a -ve shift means badblocks are
// disabled.
    pub /: *mut *mut *mut u64 page; / badblock list,
    pub changed: c_int,
    pub lock: seqlock_t,
    pub sector: sector_t,
    pub /: *mut *mut sector_t size; / in sectors,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct badblocks_context {
    pub start: sector_t,
    pub len: sector_t,
    pub ack: c_int,
}

extern "C" {
    pub fn badblocks_clear(bb: *mut badblocks, s: sector_t, sectors: sector_t) -> bool;
}
extern "C" {
    pub fn ack_all_badblocks(bb: *mut badblocks);
}
extern "C" {
    pub fn badblocks_show(bb: *mut badblocks, page: *mut c_char, unack: c_int) -> isize;
}
extern "C" {
    pub fn badblocks_init(bb: *mut badblocks, enable: c_int) -> c_int;
}
extern "C" {
    pub fn badblocks_exit(bb: *mut badblocks);
}
extern "C" {
    pub fn devm_init_badblocks(dev: *mut device, bb: *mut badblocks) -> c_int;
}
