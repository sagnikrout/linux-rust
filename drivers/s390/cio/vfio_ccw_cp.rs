//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/vfio_ccw_cp.h
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
// channel program interfaces
//
// Copyright IBM Corp. 2017
//
// Author(s): Dong Jia Shi <bjsdjshi@linux.vnet.ibm.com>
// Xiao Feng Ren <renxiaof@linux.vnet.ibm.com>
//

//
// Max length for ccw chain.
// XXX: Limit to 256, need to check more?
//
pub const CCWCHAIN_LEN_MAX: c_int = 256;
//
// Maximum number of chains
//
pub const CCWCHAIN_COUNT_MAX: c_int = 16;
//
// struct channel_program - manage information for channel program
// @ccwchain_list: list head of ccwchains
// @orb: orb for the currently processed ssch request
// @initialized: whether this instance is actually initialized
// @guest_cp: copy of guest channel program
// @ccwchain_count: number of channel program segments (linked by TIC)
// @guest_iova: first data address of a guest channel program
//
// @ccwchain_list is the head of a ccwchain list, that contents the
// translated result of the guest channel program that pointed out by
// the iova parameter when calling cp_init.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_program {
    pub ccwchain_list: list_head,
    pub orb: orb,
    pub initialized: bool,
    pub guest_cp: *mut ccw1,
    pub ccwchain_count: c_uint,
    pub guest_iova: u64,
}

extern "C" {
    pub fn cp_init(cp: *mut channel_program, orb: *mut orb) -> c_int;
}
extern "C" {
    pub fn cp_free(cp: *mut channel_program);
}
extern "C" {
    pub fn cp_prefetch(cp: *mut channel_program) -> c_int;
}
extern "C" {
    pub fn cp_update_scsw(cp: *mut channel_program, scsw: *mut scsw);
}
extern "C" {
    pub fn cp_iova_pinned(cp: *mut channel_program, iova: u64, length: u64) -> bool;
}
