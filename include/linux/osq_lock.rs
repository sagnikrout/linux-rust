//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/osq_lock.h
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
// An MCS like lock especially tailored for optimistic spinning for sleeping
// lock implementations (mutex, rwsem, etc).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optimistic_spin_queue {
//
// Stores an encoded value of the CPU # of the tail node in the queue.
// If the queue is empty, then it's set to OSQ_UNLOCKED_VAL.
//
    pub tail: core::sync::atomic::AtomicI32,
}

// Init macro and function.

extern "C" {
    pub fn osq_lock(lock: *mut optimistic_spin_queue) -> bool;
}
extern "C" {
    pub fn osq_unlock(lock: *mut optimistic_spin_queue);
}
