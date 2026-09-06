//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/percpu_counter.h
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
// A simple "approximate counter" for use in ext2 and ext3 superblocks.
//
// WARNING: these things are HUGE.  4 kbytes per counter on 32-way P4.
//

// percpu_counter batch for local add or sub

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_counter {
    pub lock: raw_spinlock_t,
    pub count: i64,

    pub /: *mut *mut list_head list; / All percpu_counters are on a list,

    pub counters: *mut s32 __percpu,
}

extern "C" {
    pub fn percpu_counter_destroy_many(fbc: *mut percpu_counter, nr_counters: u32);
}
extern "C" {
    pub fn percpu_counter_set(fbc: *mut percpu_counter, amount: i64);
}
extern "C" {
    pub fn __percpu_counter_sum(fbc: *mut percpu_counter) -> i64;
}
extern "C" {
    pub fn __percpu_counter_compare(fbc: *mut percpu_counter, rhs: i64, batch: i32) -> c_int;
}
extern "C" {
    pub fn percpu_counter_sync(fbc: *mut percpu_counter);
}
extern "C" {
    pub fn __percpu_counter_compare(_arg: fbc, _arg: rhs, _arg: percpu_counter_batch) -> return;
}
//
// With percpu_counter_add_local() and percpu_counter_sub_local(), counts
// are accumulated in local per cpu counter and not in fbc->count until
// local count overflows PERCPU_COUNTER_LOCAL_BATCH. This makes counter
// write efficient.
// But percpu_counter_sum(), instead of percpu_counter_read(), needs to be
// used to add up the counts from each CPU to account for all the local
// counts. So percpu_counter_add_local() and percpu_counter_sub_local()
// should be used when a counter is updated frequently and read rarely.
//
extern "C" {
    pub fn __percpu_counter_sum(_arg: fbc) -> return;
}
//
// It is possible for the percpu_counter_read() to return a small negative
// number for some counter which should never be negative.
//
// Prevent reloads of fbc->count

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_counter {
    pub count: i64,
}

extern "C" {
    pub fn percpu_counter_init_many(_arg: fbc, _arg: amount, _arg: gfp, _arg: 1) -> return;
}
extern "C" {
    pub fn percpu_counter_compare(_arg: fbc, _arg: rhs) -> return;
}
// non-SMP percpu_counter_add_local is the same with percpu_counter_add
//
// percpu_counter is intended to track positive numbers. In the UP case the
// number should never be negative.
//
extern "C" {
    pub fn percpu_counter_read_positive(_arg: fbc) -> return;
}
extern "C" {
    pub fn percpu_counter_read(_arg: fbc) -> return;
}

