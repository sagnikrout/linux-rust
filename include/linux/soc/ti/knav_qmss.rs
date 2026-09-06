//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/ti/knav_qmss.h
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
// Keystone Navigator Queue Management Sub-System header
//
// Copyright (C) 2014 Texas Instruments Incorporated - https://www.ti.com
// Author:	Sandeep Nair <sandeep_n@ti.com>
// Cyril Chemparathy <cyril@ti.com>
// Santosh Shilimkar <santosh.shilimkar@ti.com>
//

// queue types

// queue flags
pub const KNAV_QUEUE_SHARED: c_uint = 0x0001		/* Queue can be shared */;
//
// enum knav_queue_ctrl_cmd -	queue operations.
// @KNAV_QUEUE_GET_ID:		Get the ID number for an open queue
// @KNAV_QUEUE_FLUSH:		forcibly empty a queue if possible
// @KNAV_QUEUE_SET_NOTIFIER:	Set a notifier callback to a queue handle.
// @KNAV_QUEUE_ENABLE_NOTIFY:	Enable notifier callback for a queue handle.
// @KNAV_QUEUE_DISABLE_NOTIFY:	Disable notifier callback for a queue handle.
// @KNAV_QUEUE_GET_COUNT:	Get number of queues.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum knav_queue_ctrl_cmd {
    KNAV_QUEUE_GET_ID,
    KNAV_QUEUE_FLUSH,
    KNAV_QUEUE_SET_NOTIFIER,
    KNAV_QUEUE_ENABLE_NOTIFY,
    KNAV_QUEUE_DISABLE_NOTIFY,
    KNAV_QUEUE_GET_COUNT
}

// Queue notifier callback prototype
extern "C" {
    pub fn void(arg: *mut *mut knav_queue_notify_fn)(void) -> typedef;
}
//
// struct knav_queue_notify_config:	Notifier configuration
// @fn:					Notifier function
// @fn_arg:				Notifier function arguments
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_queue_notify_config {
    pub fn: knav_queue_notify_fn,
    pub fn_arg: *mut c_void,
}

extern "C" {
    pub fn knav_queue_close(qhandle: *mut c_void);
}
extern "C" {
    pub fn knav_queue_pop(qhandle: *mut c_void, size: *mut unsigned) -> dma_addr_t;
}
extern "C" {
    pub fn knav_pool_destroy(ph: *mut c_void);
}
extern "C" {
    pub fn knav_pool_count(ph: *mut c_void) -> c_int;
}
extern "C" {
    pub fn knav_pool_desc_put(ph: *mut c_void, desc: *mut c_void);
}
extern "C" {
    pub fn knav_pool_desc_virt_to_dma(ph: *mut c_void, virt: *mut c_void) -> dma_addr_t;
}
extern "C" {
    pub fn knav_qmss_device_ready() -> bool;
}
