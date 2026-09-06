//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/idle.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpu_idle_type {
    __CPU_NOT_IDLE = 0,
    CPU_IDLE,
    CPU_NEWLY_IDLE,
    CPU_MAX_IDLE_TYPES
}

extern "C" {
    pub fn wake_up_if_idle(cpu: c_int);
}
//
// Idle thread specific functions to determine the need_resched
// polling state.
//

//
// Polling state must be visible before we test NEED_RESCHED,
// paired by resched_curr()
//
extern "C" {
    pub fn unlikely(_arg: tif_need_resched()) -> return;
}
//
// Polling state must be visible before we test NEED_RESCHED,
// paired by resched_curr()
//
extern "C" {
    pub fn unlikely(_arg: tif_need_resched()) -> return;
}
//
// Ensure we check TIF_NEED_RESCHED after we clear the polling bit.
// Once the bit is cleared, we'll get IPIs with every new
// TIF_NEED_RESCHED and the IPI handler, scheduler_ipi(), will also
// fold.
//

extern "C" {
    pub fn unlikely(_arg: tif_need_resched()) -> return;
}
extern "C" {
    pub fn unlikely(_arg: tif_need_resched()) -> return;
}

