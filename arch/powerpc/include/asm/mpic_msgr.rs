//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/mpic_msgr.h
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
// Copyright 2011-2012, Meador Inge, Mentor Graphics Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpic_msgr {
    pub base: *mut u32 __iomem,
    pub mer: *mut u32 __iomem,
    pub irq: c_int,
    pub in_use: c_uchar,
    pub lock: raw_spinlock_t,
    pub num: c_int,
}

// Get a message register
//
// @reg_num:	the MPIC message register to get
//
// A pointer to the message register is returned.  If
// the message register asked for is already in use, then
// EBUSY is returned.  If the number given is not associated
// with an actual message register, then ENODEV is returned.
// Successfully getting the register marks it as in use.
//
// Relinquish a message register
//
// @msgr:	the message register to return
//
// Disables the given message register and marks it as free.
// After this call has completed successully the message
// register is available to be acquired by a call to
// mpic_msgr_get.
//
extern "C" {
    pub fn mpic_msgr_put(msgr: *mut mpic_msgr);
}
// Enable a message register
//
// @msgr:	the message register to enable
//
// The given message register is enabled for sending
// messages.
//
extern "C" {
    pub fn mpic_msgr_enable(msgr: *mut mpic_msgr);
}
// Disable a message register
//
// @msgr:	the message register to disable
//
// The given message register is disabled for sending
// messages.
//
extern "C" {
    pub fn mpic_msgr_disable(msgr: *mut mpic_msgr);
}
// Write a message to a message register
//
// @msgr:	the message register to write to
// @message:	the message to write
//
// The given 32-bit message is written to the given message
// register.  Writing to an enabled message registers fires
// an interrupt.
//
// Read a message from a message register
//
// @msgr:	the message register to read from
//
// Returns the 32-bit value currently in the given message register.
// Upon reading the register any interrupts for that register are
// cleared.
//
extern "C" {
    pub fn in_be32(_arg: msgr->base) -> return;
}
// Clear a message register
//
// @msgr:	the message register to clear
//
// Clears any interrupts associated with the given message register.
//
// Set the destination CPU for the message register
//
// @msgr:	the message register whose destination is to be set
// @cpu_num:	the Linux CPU number to bind the message register to
//
// Note that the CPU number given is the CPU number used by the kernel
// and *not* the actual hardware CPU number.
//
// Get the IRQ number for the message register
// @msgr:	the message register whose IRQ is to be returned
//
// Returns the IRQ number associated with the given message register.
// 0 is returned if this message register is not capable of receiving
// interrupts.  What message register can and cannot receive interrupts is
// specified in the device tree for the system.
//
