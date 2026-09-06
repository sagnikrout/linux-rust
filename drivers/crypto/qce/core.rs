//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/qce/core.h
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
// Copyright (c) 2010-2014, The Linux Foundation. All rights reserved.
//

//
// struct qce_device - crypto engine device structure
// @queue: crypto request queue
// @lock: the lock protects queue and req
// @done_work: workqueue context
// @req: current active request
// @result: result of current transform
// @base: virtual IO base
// @dev: pointer to device structure
// @core: core device clock
// @iface: interface clock
// @bus: bus clock
// @dma: pointer to dma data
// @burst_size: the crypto burst size
// @pipe_pair_id: which pipe pair id the device using
// @async_req_enqueue: invoked by every algorithm to enqueue a request
// @async_req_done: invoked by every algorithm to finish its request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qce_device {
    pub queue: crypto_queue,
    pub lock: mutex,
    pub done_work: work_struct,
    pub req: *mut crypto_async_request,
    pub result: c_int,
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub bus: *mut *mut *mut clk core, iface,,
    pub mem_path: *mut icc_path,
    pub dma: qce_dma_data,
    pub burst_size: c_int,
    pub pipe_pair_id: c_uint,
    pub req): *mut crypto_async_request,
    pub ret): *mut *mut *mut void (async_req_done)(struct qce_device qce, int,
}

//
// struct qce_algo_ops - algorithm operations per crypto type
// @type: should be CRYPTO_ALG_TYPE_XXX
// @register_algs: invoked by core to register the algorithms
// @unregister_algs: invoked by core to unregister the algorithms
// @async_req_handle: invoked by core to handle enqueued request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qce_algo_ops {
    pub type: u32,
    pub qce): *mut *mut int (register_algs)(struct qce_device,
    pub qce): *mut *mut void (unregister_algs)(struct qce_device,
    pub async_req): *mut *mut int (async_req_handle)(struct crypto_async_request,
}
