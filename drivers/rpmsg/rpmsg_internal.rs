//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/rpmsg/rpmsg_internal.h
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
// remote processor messaging bus internals
//
// Copyright (C) 2011 Texas Instruments, Inc.
// Copyright (C) 2011 Google, Inc.
//
// Ohad Ben-Cohen <ohad@wizery.com>
// Brian Swetland <swetland@google.com>
//

//
// struct rpmsg_device_ops - indirection table for the rpmsg_device operations
// @create_channel:	create backend-specific channel, optional
// @release_channel:	release backend-specific channel, optional
// @create_ept:		create backend-specific endpoint, required
// @announce_create:	announce presence of new channel, optional
// @announce_destroy:	announce destruction of channel, optional
//
// Indirection table for the operations that a rpmsg backend should implement.
// @announce_create and @announce_destroy are optional as the backend might
// advertise new channels implicitly by creating the endpoints.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_device_ops {
    pub chinfo): *mut rpmsg_channel_info,
    pub chinfo): *mut rpmsg_channel_info,
    pub chinfo): rpmsg_channel_info,
    pub rpdev): *mut *mut int (announce_create)(struct rpmsg_device,
    pub rpdev): *mut *mut int (announce_destroy)(struct rpmsg_device,
}

//
// struct rpmsg_endpoint_ops - indirection table for rpmsg_endpoint operations
// @destroy_ept:	see @rpmsg_destroy_ept(), required
// @send:		see @rpmsg_send(), required
// @sendto:		see @rpmsg_sendto(), optional
// @trysend:		see @rpmsg_trysend(), required
// @trysendto:		see @rpmsg_trysendto(), optional
// @poll:		see @rpmsg_poll(), optional
// @set_flow_control:	see @rpmsg_set_flow_control(), optional
// @get_mtu:		see @rpmsg_get_mtu(), optional
//
// Indirection table for the operations that a rpmsg backend should implement.
// In addition to @destroy_ept, the backend must at least implement @send and
// @trysend, while the variants sending data off-channel are optional.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_endpoint_ops {
    pub ept): *mut *mut void (destroy_ept)(struct rpmsg_endpoint,
    pub len): *const *const *const *const int (send)(struct rpmsg_endpoint ept, void data, int,
    pub dst): *const *const *const *const int (sendto)(struct rpmsg_endpoint ept, void data, int len, u32,
    pub len): *const *const *const *const int (trysend)(struct rpmsg_endpoint ept, void data, int,
    pub dst): *const *const *const *const int (trysendto)(struct rpmsg_endpoint ept, void data, int len, u32,
    pub wait): *mut poll_table,
    pub dst): *mut *mut *mut int (set_flow_control)(struct rpmsg_endpoint ept, bool pause, u32,
    pub ept): *mut *mut ssize_t (get_mtu)(struct rpmsg_endpoint,
}

//
// rpmsg_ctrldev_register_device() - register a char device for control based on rpdev
// @rpdev:	prepared rpdev to be used for creating endpoints
//
// This function wraps rpmsg_register_device() preparing the rpdev for use as
// basis for the rpmsg chrdev.
//
extern "C" {
    pub fn rpmsg_register_device_override(_arg: rpdev, _arg: "rpmsg_ctrl") -> return;
}
