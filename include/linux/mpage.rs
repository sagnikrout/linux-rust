//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mpage.h
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
// include/linux/mpage.h
//
// Contains declarations related to preparing and submitting BIOS which contain
// multiple pagecache pages.
//
// (And no, it doesn't do the #ifdef __MPAGE_H thing, and it doesn't do
// nested includes.  Get it right in the .c file).
//

extern "C" {
    pub fn mpage_readahead(: *mut readahead_control, get_block: get_block_t);
}
extern "C" {
    pub fn mpage_read_folio(folio: *mut folio, get_block: get_block_t) -> c_int;
}
extern "C" {
    pub fn __mpage_writepages(_arg: mapping, _arg: wbc, _arg: get_block, _arg: NULL) -> return;
}
