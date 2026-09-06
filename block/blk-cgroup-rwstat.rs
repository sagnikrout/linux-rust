//! Automatically rewritten from C Header to Rust Module
//! Source: block/blk-cgroup-rwstat.h
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
// Legacy blkg rwstat helpers enabled by CONFIG_BLK_CGROUP_RWSTAT.
// Do not use in new code.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blkg_rwstat_type {
    BLKG_RWSTAT_READ,
    BLKG_RWSTAT_WRITE,
    BLKG_RWSTAT_SYNC,
    BLKG_RWSTAT_ASYNC,
    BLKG_RWSTAT_DISCARD,

    BLKG_RWSTAT_NR,
    BLKG_RWSTAT_TOTAL = BLKG_RWSTAT_NR,
}

//
// blkg_[rw]stat->aux_cnt is excluded for local stats but included for
// recursive.  Used to carry stats of dead children.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkg_rwstat {
    pub cpu_cnt: [percpu_counter; BLKG_RWSTAT_NR],
    pub aux_cnt: [core::sync::atomic::AtomicI64; BLKG_RWSTAT_NR],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkg_rwstat_sample {
    pub cnt: [u64; BLKG_RWSTAT_NR],
}

extern "C" {
    pub fn blkg_rwstat_init(rwstat: *mut blkg_rwstat, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn blkg_rwstat_exit(rwstat: *mut blkg_rwstat);
}
//
// blkg_rwstat_add - add a value to a blkg_rwstat
// @rwstat: target blkg_rwstat
// @opf: REQ_OP and flags
// @val: value to add
//
// Add @val to @rwstat.  The counters are chosen according to @rw.  The
// caller is responsible for synchronizing calls to this function.
//
// blkg_rwstat_read - read the current values of a blkg_rwstat
// @rwstat: blkg_rwstat to read
// @result: where to put the current values
//
// Read the current snapshot of @rwstat and return it in the @result counts.
//
// blkg_rwstat_total - read the total count of a blkg_rwstat
// @rwstat: blkg_rwstat to read
//
// Return the total count of @rwstat regardless of the IO direction.  This
// function can be called without synchronization and takes care of u64
// atomicity.
//
// blkg_rwstat_reset - reset a blkg_rwstat
// @rwstat: blkg_rwstat to reset
//
// blkg_rwstat_add_aux - add a blkg_rwstat into another's aux count
// @to: the destination blkg_rwstat
// @from: the source
//
// Add @from's count including the aux one to @to's aux count.
//
