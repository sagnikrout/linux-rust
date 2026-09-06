//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/devm-helpers.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Functions which do automatically cancel operations or release resources upon
// driver detach.
//
// These should be helpful to avoid mixing the manual and devm-based resource
// management which can be source of annoying, rarely occurring,
// hard-to-reproduce bugs.
//
// Please take into account that devm based cancellation may be performed some
// time after the remove() is ran.
//
// Thus mixing devm and manual resource management can easily cause problems
// when unwinding operations with dependencies. IRQ scheduling a work in a queue
// is typical example where IRQs are often devm-managed and WQs are manually
// cleaned at remove(). If IRQs are not manually freed at remove() (and this is
// often the case when we use devm for IRQs) we have a period of time after
// remove() - and before devm managed IRQs are freed - where new IRQ may fire
// and schedule a work item which won't be cancelled because remove() was
// already ran.
//

//
// devm_delayed_work_autocancel - Resource-managed delayed work allocation
// @dev:	Device which lifetime work is bound to
// @w:		Work item to be queued
// @worker:	Worker function
//
// Initialize delayed work which is automatically cancelled when driver is
// detached. A few drivers need delayed work which must be cancelled before
// driver is detached to avoid accessing removed resources.
// devm_delayed_work_autocancel() can be used to omit the explicit
// cancellation when driver is detached.
//
extern "C" {
    pub fn devm_add_action(_arg: dev, _arg: devm_delayed_work_drop, _arg: w) -> return;
}
//
// devm_work_autocancel - Resource-managed work allocation
// @dev:	Device which lifetime work is bound to
// @w:		Work to be added (and automatically cancelled)
// @worker:	Worker function
//
// Initialize work which is automatically cancelled when driver is detached.
// A few drivers need to queue work which must be cancelled before driver
// is detached to avoid accessing removed resources.
// devm_work_autocancel() can be used to omit the explicit
// cancellation when driver is detached.
//
extern "C" {
    pub fn devm_add_action(_arg: dev, _arg: devm_work_drop, _arg: w) -> return;
}
