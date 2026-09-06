//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/block/scm_blk.h
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

pub const SCM_NR_PARTS: c_int = 8;
pub const SCM_QUEUE_DELAY: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_blk_dev {
    pub rq: *mut request_queue,
    pub gendisk: *mut gendisk,
    pub tag_set: blk_mq_tag_set,
    pub scmdev: *mut scm_device,
    pub lock: spinlock_t,
    pub queued_reqs: core::sync::atomic::AtomicI32,
    pub state: {SCM_OPER, SCM_WR_PROHIBIT},
    pub finished_requests: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_request {
    pub bdev: *mut scm_blk_dev,
    pub next_aidaw: *mut aidaw,
    pub request: *mut request,
    pub aob: *mut aob,
    pub list: list_head,
    pub retries: u8,
    pub error: blk_status_t,
}

extern "C" {
    pub fn scm_blk_dev_setup(: *mut scm_blk_dev, : *mut scm_device) -> c_int;
}
extern "C" {
    pub fn scm_blk_dev_cleanup(: *mut scm_blk_dev);
}
extern "C" {
    pub fn scm_blk_set_available(: *mut scm_blk_dev);
}
extern "C" {
    pub fn scm_blk_irq(: *mut scm_device, : *mut c_void, _arg: blk_status_t);
}
extern "C" {
    pub fn scm_drv_init() -> c_int;
}
extern "C" {
    pub fn scm_drv_cleanup();
}

