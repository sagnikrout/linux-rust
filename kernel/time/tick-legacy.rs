//! Automatically rewritten from C to Rust
//! Source: kernel/time/tick-legacy.c
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
// Timer tick function for architectures that lack generic clockevents,
// consolidated here from m68k/ia64/parisc/arm.
//

//
// legacy_timer_tick() - advances the timekeeping infrastructure
// @ticks:	number of ticks, that have elapsed since the last call.
//
// This is used by platforms that have not been converted to
// generic clockevents.
//
// If 'ticks' is zero, the CPU is not handling timekeeping, so
// only perform process accounting and profiling.
//
// Must be called with interrupts disabled.
//
#[no_mangle]
pub unsafe extern "C" fn legacy_timer_tick(ticks: c_ulong) {
    void legacy_timer_tick(unsigned long ticks)
    {
    if (ticks) {
    raw_spin_lock(&jiffies_lock);
    write_seqcount_begin(&jiffies_seq);
    do_timer(ticks);
    write_seqcount_end(&jiffies_seq);
    raw_spin_unlock(&jiffies_lock);
    update_wall_time();
    }
    update_process_times(user_mode(get_irq_regs()));
    profile_tick(CPU_PROFILING);
    }
