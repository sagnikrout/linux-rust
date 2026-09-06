//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/spinlock.h
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
// S390 version
// Copyright IBM Corp. 1999
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
//
// Derived from "include/asm-i386/spinlock.h"
//

extern "C" {
    pub fn arch_vcpu_is_preempted(cpu: c_int) -> bool;
}

//
// Simple spin lock operations.  There are two variants, one clears IRQ's
// on the local processor, one does not.
//
// We make no fairness assumptions. They have a cost.
//
// (the type definitions are in asm/spinlock_types.h)
//
extern "C" {
    pub fn arch_spin_relax(lock: *mut arch_spinlock_t);
}

extern "C" {
    pub fn arch_spin_lock_wait(: *mut arch_spinlock_t);
}
extern "C" {
    pub fn arch_spin_trylock_retry(: *mut arch_spinlock_t) -> c_int;
}
extern "C" {
    pub fn arch_spin_lock_setup(cpu: c_int);
}
extern "C" {
    pub fn likely(_arg: arch_try_cmpxchg(&lp->lock, _arg: &old, _arg: spinlock_lockval())) -> return;
}
extern "C" {
    pub fn arch_spin_trylock_retry(_arg: lp) -> return;
}
//
// Read-write spinlocks, allowing multiple readers
// but only one writer.
//
// NOTE! it is quite common to have readers in interrupts
// but no interrupt writers. For those circumstances we
// can "mix" irq-safe locks - any writer needs to get a
// irq-safe write-lock, but readers can get non-irqsafe
// read-locks.
//

extern "C" {
    pub fn arch_read_lock_wait(lp: *mut arch_rwlock_t);
}
extern "C" {
    pub fn arch_write_lock_wait(lp: *mut arch_rwlock_t);
}
