//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/entry-virt.h
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

// Transfer to guest mode work

//
// arch_xfer_to_guest_mode_handle_work - Architecture specific xfer to guest
// mode work handling function.
// @vcpu:	Pointer to current's VCPU data
// @ti_work:	Cached TIF flags gathered in xfer_to_guest_mode_handle_work()
//
// Invoked from xfer_to_guest_mode_handle_work(). Defaults to NOOP. Can be
// replaced by architecture specific code.
//
extern "C" {
    pub fn arch_xfer_to_guest_mode_handle_work(ti_work: c_ulong) -> c_int;
}

//
// xfer_to_guest_mode_handle_work - Check and handle pending work which needs
// to be handled before going to guest mode
//
// Returns: 0 or an error code
//
extern "C" {
    pub fn xfer_to_guest_mode_handle_work() -> c_int;
}
//
// xfer_to_guest_mode_prepare - Perform last minute preparation work that
// need to be handled while IRQs are disabled
// upon entering to guest.
//
// Has to be invoked with interrupts disabled before the last call
// to xfer_to_guest_mode_work_pending().
//
// __xfer_to_guest_mode_work_pending - Check if work is pending
//
// Returns: True if work pending, False otherwise.
//
// Bare variant of xfer_to_guest_mode_work_pending(). Can be called from
// interrupt enabled code for racy quick checks with care.
//
// xfer_to_guest_mode_work_pending - Check if work is pending which needs to be
// handled before returning to guest mode
//
// Returns: True if work pending, False otherwise.
//
// Has to be invoked with interrupts disabled before the transition to
// guest mode.
//
extern "C" {
    pub fn __xfer_to_guest_mode_work_pending() -> return;
}

