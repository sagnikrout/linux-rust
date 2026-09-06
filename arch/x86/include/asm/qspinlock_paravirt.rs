//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/qspinlock_paravirt.h
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

extern "C" {
    pub fn __pv_queued_spin_unlock_slowpath(lock: *mut qspinlock, locked: u8) -> void __lockfunc;
}
//
// For x86-64, PV_CALLEE_SAVE_REGS_THUNK() saves and restores 8 64-bit
// registers. For i386, however, only 1 32-bit register needs to be saved
// and restored. So an optimized version of __pv_queued_spin_unlock() is
// hand-coded for 64-bit, but it isn't worthwhile to do it for 32-bit.
//

//
// Optimized assembly version of __raw_callee_save___pv_queued_spin_unlock
// which combines the registers saving trunk and the body of the following
// C code.  Note that it puts the code in the .spinlock.text section which
// is equivalent to adding __lockfunc in the C code:
//
// void __lockfunc __pv_queued_spin_unlock(struct qspinlock *lock)
// {
// u8 lockval = _Q_LOCKED_VAL;
//
// if (try_cmpxchg(&lock->locked, &lockval, 0))
// return;
// pv_queued_spin_unlock_slowpath(lock, lockval);
// }
//
// For x86-64,
// rdi = lock              (first argument)
// rsi = lockval           (second argument)
// rdx = internal variable (set to 0)
//

extern "C" {
    pub fn __pv_queued_spin_unlock(lock: *mut qspinlock) -> void __lockfunc;
}

