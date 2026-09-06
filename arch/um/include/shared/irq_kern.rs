//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/shared/irq_kern.h
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
// Copyright (C) 2001, 2002 Jeff Dike (jdike@karaya.com)
//

//
// um_request_irq_tt - request an IRQ with timetravel handler
//
// @irq: the IRQ number, or %UM_IRQ_ALLOC
// @fd: The file descriptor to request an IRQ for
// @type: read or write
// @handler: the (generic style) IRQ handler
// @irqflags: Linux IRQ flags
// @devname: name for this to show
// @dev_id: data pointer to pass to the IRQ handler
// @timetravel_handler: the timetravel interrupt handler, invoked with the IRQ
// number, fd, dev_id and time-travel event pointer.
//
// Returns: The interrupt number assigned or a negative error.
//
// Note that the timetravel handler is invoked only if the time_travel_mode is
// %TT_MODE_EXTERNAL, and then it is invoked even while the system is suspended!
// This function must call time_travel_add_irq_event() for the event passed with
// an appropriate delay, before sending an ACK on the socket it was invoked for.
//
// If this was called while the system is suspended, then adding the event will
// cause the system to resume.
//
// Since this function will almost certainly have to handle the FD's condition,
// a read will consume the message, and after that it is up to the code using
// it to pass such a message to the @handler in whichever way it can.
//
// If time_travel_mode is not %TT_MODE_EXTERNAL the @timetravel_handler will
// not be invoked at all and the @handler must handle the FD becoming
// readable (or writable) instead. Use um_irq_timetravel_handler_used() to
// distinguish these cases.
//
// See virtio_uml.c for an example.
//

extern "C" {
    pub fn um_free_irq(irq: c_int, dev_id: *mut c_void);
}
extern "C" {
    pub fn free_irqs();
}
