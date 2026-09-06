//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irq_poll.h
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
    pub fn int(: *mut irq_poll_fn)(struct irq_poll, _arg: c_int) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_poll {
    pub list: list_head,
    pub state: c_ulong,
    pub weight: c_int,
    pub poll: *mut irq_poll_fn,
}

extern "C" {
    pub fn irq_poll_sched(: *mut irq_poll);
}
extern "C" {
    pub fn irq_poll_init(: *mut irq_poll, _arg: c_int, : *mut irq_poll_fn);
}
extern "C" {
    pub fn irq_poll_complete(: *mut irq_poll);
}
extern "C" {
    pub fn irq_poll_enable(: *mut irq_poll);
}
extern "C" {
    pub fn irq_poll_disable(: *mut irq_poll);
}
