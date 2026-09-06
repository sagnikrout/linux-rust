//! Automatically rewritten from C to Rust
//! Source: kernel/time/timecounter.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Based on clocksource code. See commit 74d23cc704d1
//

    void timecounter_init(struct timecounter *tc,
    struct cyclecounter *cc,
    u64 start_tstamp)
    {
    tc.cc = cc;
    tc.cycle_last = cc.read(cc);
    tc.nsec = start_tstamp;
    tc.mask = (1ULL << cc.shift) - 1;
    tc.frac = 0;
    }
    EXPORT_SYMBOL_GPL(timecounter_init);
//
// timecounter_read_delta - get nanoseconds since last call of this function
// @tc:         Pointer to time counter
//
// When the underlying cycle counter runs over, this will be handled
// correctly as long as it does not run over more than once between
// calls.
//
// The first call to this function for a new time counter initializes
// the time tracking and returns an undefined result.
//
#[no_mangle]
unsafe extern "C" fn timecounter_read_delta(tc: *mut timecounter) -> u64 {
    static u64 timecounter_read_delta(struct timecounter *tc)
    {
    u64 cycle_now, cycle_delta;
    u64 ns_offset;
// read cycle counter:
    cycle_now = tc.cc.read(tc.cc);
// calculate the delta since the last timecounter_read_delta():
    cycle_delta = (cycle_now - tc.cycle_last) & tc.cc.mask;
// convert to nanoseconds:
    ns_offset = cyclecounter_cyc2ns(tc.cc, cycle_delta,
    tc.mask, &tc.frac);
// update time stamp of timecounter_read_delta() call:
    tc.cycle_last = cycle_now;
    return ns_offset;
    }
#[no_mangle]
pub unsafe extern "C" fn timecounter_read(tc: *mut timecounter) -> u64 {
    u64 timecounter_read(struct timecounter *tc)
    {
    u64 nsec;
// increment time by nanoseconds since last call
    nsec = timecounter_read_delta(tc);
    nsec += tc.nsec;
    tc.nsec = nsec;
    return nsec;
    }
    EXPORT_SYMBOL_GPL(timecounter_read);
