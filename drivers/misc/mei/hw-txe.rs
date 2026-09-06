//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/mei/hw-txe.h
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
// Copyright (c) 2013-2016, Intel Corporation. All rights reserved.
// Intel Management Engine Interface (Intel MEI) Linux driver
//

// Flatten Hierarchy interrupt cause

//
// struct mei_txe_hw - txe hardware specifics
//
// @mem_addr:            SeC and BRIDGE bars
// @aliveness:           aliveness (power gating) state of the hardware
// @readiness:           readiness state of the hardware
// @slots:               number of empty slots
// @wait_aliveness_resp: aliveness wait queue
// @intr_cause:          translated interrupt cause
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_txe_hw {
    pub mem_addr: *const *const void __iomem,
    pub aliveness: u32,
    pub readiness: u32,
    pub slots: u32,
    pub wait_aliveness_resp: wait_queue_head_t,
    pub intr_cause: c_ulong,
}

extern "C" {
    pub fn container_of()hw: *mut (void, mei_device: struct, _arg: hw) -> return;
}
extern "C" {
    pub fn mei_txe_irq_quick_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn mei_txe_irq_thread_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn mei_txe_aliveness_set_sync(dev: *mut mei_device, req: u32) -> c_int;
}
