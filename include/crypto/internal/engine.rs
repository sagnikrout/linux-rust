//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/engine.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Crypto engine API
//
// Copyright (c) 2016 Baolin Wang <baolin.wang@linaro.org>
// Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
//

pub const ENGINE_NAME_LEN: c_int = 30;
//
// struct crypto_engine - crypto hardware engine
// @name: the engine name
// @busy: request pump is busy
// @running: the engine is on working
// @retry_support: indication that the hardware allows re-execution
// of a failed backlog request
// crypto-engine, in head position to keep order
// @rt: whether this queue is set to run as a realtime task
// @list: link with the global crypto engine list
// @queue_lock: spinlock to synchronise access to request queue
// @queue: the crypto queue of the engine
// @kworker: kthread worker struct for request pump
// @pump_requests: work struct for scheduling work to the request pump
// @priv_data: the engine private data
// @cur_req: the current request which is on processing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_engine {
    pub name: [c_char; ENGINE_NAME_LEN],
    pub busy: bool,
    pub running: bool,
    pub retry_support: bool,
    pub rt: bool,
    pub list: list_head,
    pub queue_lock: spinlock_t,
    pub __guarded_by(&queue_lock): crypto_queue queue,
    pub dev: *mut device,
    pub kworker: *mut kthread_worker,
    pub pump_requests: kthread_work,
    pub priv_data: *mut c_void,
    pub cur_req: *mut crypto_async_request,
}
