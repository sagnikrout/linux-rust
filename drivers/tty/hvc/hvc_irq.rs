//! Automatically rewritten from C to Rust
//! Source: drivers/tty/hvc/hvc_irq.c
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
// Copyright IBM Corp. 2001,2008
//
// This file contains the IRQ specific code for hvc_console
//

#[no_mangle]
unsafe extern "C" fn hvc_handle_interrupt(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t {
    static irqreturn_t hvc_handle_interrupt(int irq, void *dev_instance)
    {
// if hvc_poll request a repoll, then kick the hvcd thread
    if (hvc_poll(dev_instance))
    hvc_kick();
//
// We're safe to always return IRQ_HANDLED as the hvcd thread will
// iterate through each hvc_struct.
//
    return IRQ_HANDLED;
    }
//
// For IRQ based systems these callbacks can be used
//
#[no_mangle]
pub unsafe extern "C" fn notifier_add_irq(hp: *mut hvc_struct, irq: c_int) -> c_int {
    int notifier_add_irq(struct hvc_struct *hp, int irq)
    {
    int rc;
    if (!irq) {
    hp.irq_requested = 0;
    return 0;
    }
    rc = request_irq(irq, hvc_handle_interrupt, hp.flags,
    "hvc_console", hp);
    if (!rc)
    hp.irq_requested = 1;
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn notifier_del_irq(hp: *mut hvc_struct, irq: c_int) {
    void notifier_del_irq(struct hvc_struct *hp, int irq)
    {
    if (!hp.irq_requested)
    return;
    free_irq(irq, hp);
    hp.irq_requested = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn notifier_hangup_irq(hp: *mut hvc_struct, irq: c_int) {
    void notifier_hangup_irq(struct hvc_struct *hp, int irq)
    {
    notifier_del_irq(hp, irq);
    }
