//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/simple_spinlock.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Simple spin lock operations.
//
// Copyright (C) 2001-2004 Paul Mackerras <paulus@au.ibm.com>, IBM
// Copyright (C) 2001 Anton Blanchard <anton@au.ibm.com>, IBM
// Copyright (C) 2002 Dave Engebretsen <engebret@us.ibm.com>, IBM
// Rework to support virtual processors
//
// Type of int is used as a full 64b word is not necessary.
//
// (the type definitions are in asm/simple_spinlock_types.h)
//

// use 0x800000yy when locked, where yy == CPU number

pub const LOCK_TOKEN: c_int = 1;

//
// This returns the old value in the lock, so we succeeded
// in getting the lock if the return value is 0.
//
// On a system with shared processors (that is, where a physical
// processor is multiplexed between several virtual processors),
// there is no point spinning on a lock if the holder of the lock
// isn't currently scheduled on a physical processor.  Instead
// we detect this situation and ask the hypervisor to give the
// rest of our timeslice to the lock holder.
//
// So that we can tell which virtual processor is holding a lock,
// we put 0x80000000 | smp_processor_id() in the lock when it is
// held.  Conveniently, we have a word in the paca that holds this
// value.
//

// We only yield to the hypervisor if we are in shared processor mode
extern "C" {
    pub fn splpar_spin_yield(lock: *mut arch_spinlock_t);
}
extern "C" {
    pub fn splpar_rw_yield(lock: *mut arch_rwlock_t);
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

//
// This returns the old value in the lock + 1,
// so we got a read lock if the return value is > 0.
//
// This returns the old value in the lock,
// so we got the write lock if the return value is 0.
//

