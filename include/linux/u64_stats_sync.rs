//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/u64_stats_sync.h
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
// Protect against 64-bit values tearing on 32-bit architectures. This is
// typically used for statistics read/update in different subsystems.
//
// Key points :
//
// -  Use a seqcount on 32-bit
// -  The whole thing is a no-op on 64-bit architectures.
//
// Usage constraints:
//
// 1) Write side must ensure mutual exclusion, or one seqcount update could
// be lost, thus blocking readers forever.
//
// 2) Write side must disable preemption, or a seqcount reader can preempt the
// writer and also spin forever.
//
// 3) Write side must use the _irqsave() variant if other writers, or a reader,
// can be invoked from an IRQ context. On 64bit systems this variant does not
// disable interrupts.
//
// 4) If reader fetches several counters, there is no guarantee the whole values
// are consistent w.r.t. each other (remember point #2: seqcounts are not
// used for 64bit architectures).
//
// 5) Readers are allowed to sleep or be preempted/interrupted: they perform
// pure reads.
//
// Usage :
//
// Stats producer (writer) should use following template granted it already got
// an exclusive access to counters (a lock is already taken, or per cpu
// data is used [in a non preemptable context])
//
// spin_lock_bh(...) or other synchronization to get exclusive access
// ...
// u64_stats_update_begin(&stats->syncp);
// u64_stats_add(&stats->bytes64, len); // non atomic operation
// u64_stats_inc(&stats->packets64);    // non atomic operation
// u64_stats_update_end(&stats->syncp);
//
// While a consumer (reader) should use following template to get consistent
// snapshot for each variable (but no guarantee on several ones)
//
// u64 tbytes, tpackets;
// unsigned int start;
//
// do {
// start = u64_stats_fetch_begin(&stats->syncp);
// tbytes = u64_stats_read(&stats->bytes64); // non atomic operation
// tpackets = u64_stats_read(&stats->packets64); // non atomic operation
// } while (u64_stats_fetch_retry(&stats->syncp, start));
//
// Example of use in drivers/net/loopback.c, using per_cpu containers,
// in BH disabled context.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct u64_stats_sync {

    pub seq: seqcount_t,

}

extern "C" {
    pub fn local64_read(_arg: &p->v) -> return;
}

extern "C" {
    pub fn memcpy(_arg: dst, _arg: src, _arg: len) -> return;
}

extern "C" {
    pub fn read_seqcount_begin(_arg: &syncp->seq) -> return;
}
extern "C" {
    pub fn read_seqcount_retry(_arg: &syncp->seq, _arg: start) -> return;
}

extern "C" {
    pub fn __u64_stats_fetch_begin(_arg: syncp) -> return;
}
extern "C" {
    pub fn __u64_stats_fetch_retry(_arg: syncp, _arg: start) -> return;
}
