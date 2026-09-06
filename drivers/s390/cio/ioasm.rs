//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/ioasm.h
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
// Some S390 specific IO instructions
//
extern "C" {
    pub fn stsch(schid: subchannel_id, addr: *mut schib) -> c_int;
}
extern "C" {
    pub fn msch(schid: subchannel_id, addr: *mut schib) -> c_int;
}
extern "C" {
    pub fn tsch(schid: subchannel_id, addr: *mut irb) -> c_int;
}
extern "C" {
    pub fn ssch(schid: subchannel_id, addr: *mut orb) -> c_int;
}
extern "C" {
    pub fn csch(schid: subchannel_id) -> c_int;
}
extern "C" {
    pub fn tpi(addr: *mut tpi_info) -> c_int;
}
extern "C" {
    pub fn chsc(chsc_area: *mut c_void) -> c_int;
}
extern "C" {
    pub fn rsch(schid: subchannel_id) -> c_int;
}
extern "C" {
    pub fn hsch(schid: subchannel_id) -> c_int;
}
extern "C" {
    pub fn xsch(schid: subchannel_id) -> c_int;
}
extern "C" {
    pub fn stcrw(crw: *mut crw) -> c_int;
}
