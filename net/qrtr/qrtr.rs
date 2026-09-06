//! Automatically rewritten from C Header to Rust Module
//! Source: net/qrtr/qrtr.h
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

// endpoint node id auto assignment

//
// struct qrtr_endpoint - endpoint handle
// @xmit: Callback for outgoing packets
//
// The socket buffer passed to the xmit function becomes owned by the endpoint
// driver.  As such, when the driver is done with the buffer, it should
// call kfree_skb() on failure, or consume_skb() on success.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qrtr_endpoint {
    pub skb): *mut *mut *mut int (xmit)(struct qrtr_endpoint ep, struct sk_buff,
// private: not for endpoint use
    pub node: *mut qrtr_node,
}

extern "C" {
    pub fn qrtr_endpoint_register(ep: *mut qrtr_endpoint, nid: c_uint) -> c_int;
}
extern "C" {
    pub fn qrtr_endpoint_unregister(ep: *mut qrtr_endpoint);
}
extern "C" {
    pub fn qrtr_endpoint_post(ep: *mut qrtr_endpoint, data: *const c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn qrtr_ns_init() -> c_int;
}
extern "C" {
    pub fn qrtr_ns_remove();
}
