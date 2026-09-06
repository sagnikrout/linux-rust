//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/if_tap.h
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

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

//
// Maximum times a tap device can be opened. This can be used to
// configure the number of receive queue, e.g. for multiqueue virtio.
//
pub const MAX_TAP_QUEUES: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tap_dev {
    pub dev: *mut net_device,
    pub flags: u16,
// This array tracks active taps.
    pub taps: [*mut tap_queue __rcu; MAX_TAP_QUEUES],
// This list tracks all taps (both enabled and disabled)
    pub queue_list: list_head,
    pub numvtaps: c_int,
    pub numqueues: c_int,
    pub tap_features: netdev_features_t,
    pub minor: c_int,
    pub features): *mut *mut *mut void (update_features)(struct tap_dev tap, netdev_features_t,
    pub tap): *mut *mut void (count_tx_dropped)(struct tap_dev,
    pub tap): *mut *mut void (count_rx_dropped)(struct tap_dev,
}

//
// A tap queue is the central object of tap module, it connects
// an open character device to virtual interface. There can be
// multiple queues on one interface, which map back to queues
// implemented in hardware on the underlying device.
//
// tap_proto is used to allocate queues through the sock allocation
// mechanism.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tap_queue {
    pub sk: sock,
    pub sock: socket,
    pub vnet_hdr_sz: c_int,
    pub tap: *mut tap_dev __rcu,
    pub file: *mut file,
    pub flags: c_uint,
    pub queue_index: u16,
    pub enabled: bool,
    pub next: list_head,
    pub ring: ptr_ring,
}

extern "C" {
    pub fn tap_handle_frame(pskb: *mut sk_buff) -> rx_handler_result_t;
}
extern "C" {
    pub fn tap_del_queues(tap: *mut tap_dev);
}
extern "C" {
    pub fn tap_get_minor(major: dev_t, tap: *mut tap_dev) -> c_int;
}
extern "C" {
    pub fn tap_free_minor(major: dev_t, tap: *mut tap_dev);
}
extern "C" {
    pub fn tap_queue_resize(tap: *mut tap_dev) -> c_int;
}
extern "C" {
    pub fn tap_destroy_cdev(major: dev_t, tap_cdev: *mut cdev);
}
