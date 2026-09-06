//! Automatically rewritten from C Header to Rust Module
//! Source: block/blk-rq-qos.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rq_qos_id {
    RQ_QOS_WBT,
    RQ_QOS_LATENCY,
    RQ_QOS_COST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_wait {
    pub wait: wait_queue_head_t,
    pub inflight: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_qos {
    pub ops: *const rq_qos_ops,
    pub disk: *mut gendisk,
    pub id: rq_qos_id,
    pub next: *mut rq_qos,

    pub debugfs_dir: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_qos_ops {
    pub ): *mut *mut *mut void (throttle)(struct rq_qos , struct bio,
    pub ): *mut *mut *mut *mut void (track)(struct rq_qos , struct request , struct bio,
    pub ): *mut *mut *mut *mut void (merge)(struct rq_qos , struct request , struct bio,
    pub ): *mut *mut *mut void (issue)(struct rq_qos , struct request,
    pub ): *mut *mut *mut void (requeue)(struct rq_qos , struct request,
    pub ): *mut *mut *mut void (done)(struct rq_qos , struct request,
    pub ): *mut *mut *mut void (done_bio)(struct rq_qos , struct bio,
    pub ): *mut *mut *mut void (cleanup)(struct rq_qos , struct bio,
    pub ): *mut *mut void (queue_depth_changed)(struct rq_qos,
    pub ): *mut *mut void (exit)(struct rq_qos,
    pub debugfs_attrs: *const blk_mq_debugfs_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_depth {
    pub max_depth: c_uint,
    pub scale_step: c_int,
    pub scaled_max: bool,
    pub queue_depth: c_uint,
    pub default_depth: c_uint,
}

extern "C" {
    pub fn rq_qos_id(_arg: q, _arg: RQ_QOS_WBT) -> return;
}
extern "C" {
    pub fn rq_qos_id(_arg: q, _arg: RQ_QOS_LATENCY) -> return;
}
extern "C" {
    pub fn rq_qos_del(rqos: *mut rq_qos);
}
extern "C" {
    pub fn bool(rqw: *mut acquire_inflight_cb_t)(struct rq_wait, private_data: *mut c_void) -> typedef;
}
extern "C" {
    pub fn void(rqw: *mut cleanup_cb_t)(struct rq_wait, private_data: *mut c_void) -> typedef;
}
extern "C" {
    pub fn rq_wait_inc_below(rq_wait: *mut rq_wait, limit: c_uint) -> bool;
}
extern "C" {
    pub fn rq_depth_scale_up(rqd: *mut rq_depth) -> bool;
}
extern "C" {
    pub fn rq_depth_scale_down(rqd: *mut rq_depth, hard_throttle: bool) -> bool;
}
extern "C" {
    pub fn rq_depth_calc_max_depth(rqd: *mut rq_depth) -> bool;
}
extern "C" {
    pub fn __rq_qos_cleanup(rqos: *mut rq_qos, bio: *mut bio);
}
extern "C" {
    pub fn __rq_qos_done(rqos: *mut rq_qos, rq: *mut request);
}
extern "C" {
    pub fn __rq_qos_issue(rqos: *mut rq_qos, rq: *mut request);
}
extern "C" {
    pub fn __rq_qos_requeue(rqos: *mut rq_qos, rq: *mut request);
}
extern "C" {
    pub fn __rq_qos_throttle(rqos: *mut rq_qos, bio: *mut bio);
}
extern "C" {
    pub fn __rq_qos_track(rqos: *mut rq_qos, rq: *mut request, bio: *mut bio);
}
extern "C" {
    pub fn __rq_qos_merge(rqos: *mut rq_qos, rq: *mut request, bio: *mut bio);
}
extern "C" {
    pub fn __rq_qos_done_bio(rqos: *mut rq_qos, bio: *mut bio);
}
extern "C" {
    pub fn __rq_qos_queue_depth_changed(rqos: *mut rq_qos);
}
//
// A BIO may carry BIO_QOS_* flags even if the associated request_queue
// does not have rq_qos enabled. This can happen with stacked block
// devices — for example, NVMe multipath, where it's possible that the
// bottom device has QoS enabled but the top device does not. Therefore,
// always verify that q->rq_qos is present and QoS is enabled before
// calling __rq_qos_done_bio().
//
extern "C" {
    pub fn rq_qos_exit(: *mut request_queue);
}
