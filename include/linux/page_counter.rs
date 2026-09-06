//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/page_counter.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_counter {
//
// Make sure 'usage' does not share cacheline with any other field in
// v2. The memcg->memory.usage is a hot member of struct mem_cgroup.
//
    pub usage: atomic_long_t,
    pub /: *mut *mut unsigned long failcnt; / v1-only field,
// effective memory.min and memory.min usage tracking
    pub emin: c_ulong,
    pub min_usage: atomic_long_t,
    pub children_min_usage: atomic_long_t,
// effective memory.low and memory.low usage tracking
    pub elow: c_ulong,
    pub low_usage: atomic_long_t,
    pub children_low_usage: atomic_long_t,
    pub watermark: c_ulong,
// Latest cg2 reset watermark
    pub local_watermark: c_ulong,
// Keep all the read most fields in a separete cacheline.
    pub protection_support: bool,
    pub track_failcnt: bool,
    pub min: c_ulong,
    pub low: c_ulong,
    pub high: c_ulong,
    pub max: c_ulong,
    pub parent: *mut page_counter,
    pub ____cacheline_internodealigned_in_smp: },

//
// Protection is supported only for the first counter (with id 0).
//
    pub (atomic_long_t)ATOMIC_LONG_INIT(0): counter->usage =,
    pub PAGE_COUNTER_MAX: counter->max =,
    pub parent: counter->parent =,
    pub protection_support: counter->protection_support =,
    pub false: counter->track_failcnt =,
    pub atomic_long_read(&counter->usage): return,
    pub nr_pages): *mut *mut void page_counter_cancel(struct page_counter counter, unsigned long,
    pub nr_pages): *mut *mut void page_counter_charge(struct page_counter counter, unsigned long,
    pub fail): *mut page_counter,
    pub nr_pages): *mut *mut void page_counter_uncharge(struct page_counter counter, unsigned long,
    pub nr_pages): *mut *mut void page_counter_set_min(struct page_counter counter, unsigned long,
    pub nr_pages): *mut *mut void page_counter_set_low(struct page_counter counter, unsigned long,
    pub nr_pages): WRITE_ONCE(counter->high,,
    pub nr_pages): *mut *mut int page_counter_set_max(struct page_counter counter, unsigned long,
    pub nr_pages): *mut c_ulong,
    pub page_counter_read(counter): unsigned long usage =,
//
// Update local_watermark first, so it's always <= watermark
// (modulo CPU/compiler re-ordering)
//
    pub usage: counter->local_watermark =,
    pub usage: counter->watermark =,

    pub recursive_protection): bool,

