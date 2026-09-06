//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/cec-pin.h
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
// cec-pin.h - low-level CEC pin control
//
// Copyright 2017 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

//
// struct cec_pin_ops - low-level CEC pin operations
// @read:	read the CEC pin. Returns > 0 if high, 0 if low, or an error
// if negative.
// @low:	drive the CEC pin low.
// @high:	stop driving the CEC pin. The pull-up will drive the pin
// high, unless someone else is driving the pin low.
// @enable_irq:	optional, enable the interrupt to detect pin voltage changes.
// @disable_irq: optional, disable the interrupt.
// @free:	optional. Free any allocated resources. Called when the
// adapter is deleted.
// @status:	optional, log status information.
// @read_hpd:	optional. Read the HPD pin. Returns > 0 if high, 0 if low or
// an error if negative.
// @read_5v:	optional. Read the 5V pin. Returns > 0 if high, 0 if low or
// an error if negative.
// @received:	optional. High-level CEC message callback. Allows the driver
// to process CEC messages.
//
// These operations (except for the @received op) are used by the
// cec pin framework to manipulate the CEC pin.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_pin_ops {
    pub adap): *mut *mut int (read)(struct cec_adapter,
    pub adap): *mut *mut void (low)(struct cec_adapter,
    pub adap): *mut *mut void (high)(struct cec_adapter,
    pub adap): *mut *mut bool (enable_irq)(struct cec_adapter,
    pub adap): *mut *mut void (disable_irq)(struct cec_adapter,
    pub adap): *mut *mut void (free)(struct cec_adapter,
    pub file): *mut *mut *mut void (status)(struct cec_adapter adap, struct seq_file,
    pub adap): *mut *mut int (read_hpd)(struct cec_adapter,
    pub adap): *mut *mut int (read_5v)(struct cec_adapter,
// High-level CEC message callback
    pub msg): *mut *mut *mut int (received)(struct cec_adapter adap, struct cec_msg,
}

//
// cec_pin_changed() - update pin state from interrupt
//
// @adap:	pointer to the cec adapter
// @value:	when true the pin is high, otherwise it is low
//
// If changes of the CEC voltage are detected via an interrupt, then
// cec_pin_changed is called from the interrupt with the new value.
//
extern "C" {
    pub fn cec_pin_changed(adap: *mut cec_adapter, value: bool);
}
//
// cec_pin_allocate_adapter() - allocate a pin-based cec adapter
//
// @pin_ops:	low-level pin operations
// @priv:	will be stored in adap->priv and can be used by the adapter ops.
// Use cec_get_drvdata(adap) to get the priv pointer.
// @name:	the name of the CEC adapter. Note: this name will be copied.
// @caps:	capabilities of the CEC adapter. This will be ORed with
// CEC_CAP_MONITOR_ALL and CEC_CAP_MONITOR_PIN.
//
// Allocate a cec adapter using the cec pin framework.
//
// Return: a pointer to the cec adapter or an error pointer
//
