//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/idset.h
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
// Copyright IBM Corp. 2007, 2012
// Author(s): Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
//

extern "C" {
    pub fn idset_free(set: *mut idset);
}
extern "C" {
    pub fn idset_fill(set: *mut idset);
}
extern "C" {
    pub fn idset_sch_add(set: *mut idset, id: subchannel_id);
}
extern "C" {
    pub fn idset_sch_del(set: *mut idset, id: subchannel_id);
}
extern "C" {
    pub fn idset_sch_del_subseq(set: *mut idset, schid: subchannel_id);
}
extern "C" {
    pub fn idset_sch_contains(set: *mut idset, id: subchannel_id) -> c_int;
}
extern "C" {
    pub fn idset_is_empty(set: *mut idset) -> c_int;
}
extern "C" {
    pub fn idset_add_set(to: *mut idset, from: *mut idset);
}
