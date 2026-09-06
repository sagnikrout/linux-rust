//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/local_lock.h
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
// local_lock_init - Runtime initialize a lock instance
// @lock:	The lock variable
//

//
// local_lock - Acquire a per CPU local lock
// @lock:	The lock variable
//

//
// local_lock_irq - Acquire a per CPU local lock and disable interrupts
// @lock:	The lock variable
//

//
// local_lock_irqsave - Acquire a per CPU local lock, save and disable
// interrupts
// @lock:	The lock variable
// @flags:	Storage for interrupt flags
//

//
// local_unlock - Release a per CPU local lock
// @lock:	The lock variable
//

//
// local_unlock_irq - Release a per CPU local lock and enable interrupts
// @lock:	The lock variable
//

//
// local_unlock_irqrestore - Release a per CPU local lock and restore
// interrupt flags
// @lock:	The lock variable
// @flags:      Interrupt flags to restore
//

//
// local_trylock_init - Runtime initialize a lock instance
// @lock:	The lock variable
//

//
// local_trylock - Try to acquire a per CPU local lock
// @lock:	The lock variable
//
// The function can be used in any context such as NMI or HARDIRQ. Due to
// locking constrains it will _always_ fail to acquire the lock in NMI or
// HARDIRQ context on PREEMPT_RT.
//

//
// local_trylock_irqsave - Try to acquire a per CPU local lock, save and disable
// interrupts if acquired
// @lock:	The lock variable
// @flags:	Storage for interrupt flags
//
// The function can be used in any context such as NMI or HARDIRQ. Due to
// locking constrains it will _always_ fail to acquire the lock in NMI or
// HARDIRQ context on PREEMPT_RT.
//

