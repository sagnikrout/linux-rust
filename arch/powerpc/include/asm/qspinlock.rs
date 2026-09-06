//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/qspinlock.h
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
// Use the EH=1 hint for accesses that result in the lock being acquired.
// The hardware is supposed to optimise this pattern by holding the lock
// cacheline longer, and releasing when a store to the same memory (the
// unlock) is performed.
//
pub const _Q_SPIN_EH_HINT: c_int = 1;

pub const _Q_SPIN_EH_HINT: c_int = 0;

//
// The trylock itself may steal. This makes trylocks slightly stronger, and
// makes locks slightly more efficient when stealing.
//
// This is compile-time, so if true then there may always be stealers, so the
// nosteal paths become unused.
//
pub const _Q_SPIN_TRY_LOCK_STEAL: c_int = 1;
//
// Put a speculation barrier after testing the lock/node and finding it
// busy. Try to prevent pointless speculation in slow paths.
//
// Slows down the lockstorm microbenchmark with no stealing, where locking
// is purely FIFO through the queue. May have more benefit in real workload
// where speculating into the wrong place could have a greater cost.
//
pub const _Q_SPIN_SPEC_BARRIER: c_int = 0;

//
// Execute a miso instruction after passing the MCS lock ownership to the
// queue head. Miso is intended to make stores visible to other CPUs sooner.
//
// This seems to make the lockstorm microbenchmark nospin test go slightly
// faster on POWER10, but disable for now.
//
pub const _Q_SPIN_MISO: c_int = 0;

pub const _Q_SPIN_MISO: c_int = 0;

//
// This executes miso after an unlock of the lock word, having ownership
// pass to the next CPU sooner. This will slow the uncontended path to some
// degree. Not evidence it helps yet.
//
pub const _Q_SPIN_MISO_UNLOCK: c_int = 0;

pub const _Q_SPIN_MISO_UNLOCK: c_int = 0;

//
// Seems to slow down lockstorm microbenchmark, suspect queue node just
// has to become shared again right afterwards when its waiter spins on
// the lock field.
//
pub const _Q_SPIN_PREFETCH_NEXT: c_int = 0;
extern "C" {
    pub fn READ_ONCE(_arg: lock->val) -> return;
}
// XXX: make this use lock value in paca like simple spinlocks?
// Trylock succeeds only when unlocked and no queued nodes
extern "C" {
    pub fn likely(0: prev ==) -> return;
}
// Trylock may get ahead of queued nodes if it finds unlocked
extern "C" {
    pub fn likely(~_Q_TAIL_CPU_MASK): !(prev &) -> return;
}
extern "C" {
    pub fn __queued_spin_trylock_nosteal(_arg: lock) -> return;
}
extern "C" {
    pub fn __queued_spin_trylock_steal(_arg: lock) -> return;
}
extern "C" {
    pub fn queued_spin_lock_slowpath(lock: *mut qspinlock);
}
extern "C" {
    pub fn volatile("memory": "miso" :::) -> asm;
}

extern "C" {
    pub fn pv_spinlocks_init();
}

