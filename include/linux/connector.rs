//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/connector.h
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
// connector.h
//
// 2004-2005 Copyright (c) Evgeniy Polyakov <zbr@ioremap.net>
// All rights reserved.
//

pub const CN_CBQ_NAMELEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn_queue_dev {
    pub refcnt: core::sync::atomic::AtomicI32,
    pub name: [c_uchar; CN_CBQ_NAMELEN],
    pub queue_list: list_head,
    pub queue_lock: spinlock_t,
    pub nls: *mut sock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn_callback_id {
    pub name: [c_uchar; CN_CBQ_NAMELEN],
    pub id: cb_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn_callback_entry {
    pub callback_entry: list_head,
    pub refcnt: refcount_t,
    pub pdev: *mut cn_queue_dev,
    pub id: cn_callback_id,
    pub ): *mut *mut *mut void (callback) (struct cn_msg , struct netlink_skb_parms,
    pub group: u32 seq,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn_dev {
    pub id: cb_id,
    pub groups: u32 seq,,
    pub nls: *mut sock,
    pub cbdev: *mut cn_queue_dev,
}

//
// cn_add_callback() - Registers new callback with connector core.
//
// @id:		unique connector's user identifier.
// It must be registered in connector.h for legal
// in-kernel users.
// @name:	connector's callback symbolic name.
// @callback:	connector's callback.
// parameters are %cn_msg and the sender's credentials
//
// cn_del_callback() - Unregisters new callback with connector core.
//
// @id:		unique connector's user identifier.
//
extern "C" {
    pub fn cn_del_callback(id: *const cb_id);
}
//
// cn_netlink_send_mult - Sends message to the specified groups.
//
// @msg: 	message header(with attached data).
// @len:	Number of @msg to be sent.
// @portid:	destination port.
// If non-zero the message will be sent to the given port,
// which should be set to the original sender.
// @group:	destination group.
// If @portid and @group is zero, then appropriate group will
// be searched through all registered connector users, and
// message will be delivered to the group which was created
// for user with the same ID as in @msg.
// If @group is not zero, then message will be delivered
// to the specified group.
// @gfp_mask:	GFP mask.
// @filter:     Filter function to be used at netlink layer.
// @filter_data:Filter data to be supplied to the filter function
//
// It can be safely called from softirq context, but may silently
// fail under strong memory pressure.
//
// If there are no listeners for given group %-ESRCH can be returned.
//
// cn_netlink_send - Sends message to the specified groups.
//
// @msg:	message header(with attached data).
// @portid:	destination port.
// If non-zero the message will be sent to the given port,
// which should be set to the original sender.
// @group:	destination group.
// If @portid and @group is zero, then appropriate group will
// be searched through all registered connector users, and
// message will be delivered to the group which was created
// for user with the same ID as in @msg.
// If @group is not zero, then message will be delivered
// to the specified group.
// @gfp_mask:	GFP mask.
//
// It can be safely called from softirq context, but may silently
// fail under strong memory pressure.
//
// If there are no listeners for given group %-ESRCH can be returned.
//
extern "C" {
    pub fn cn_netlink_send(msg: *mut cn_msg, portid: u32, group: u32, gfp_mask: gfp_t) -> c_int;
}
extern "C" {
    pub fn cn_queue_del_callback(dev: *mut cn_queue_dev, id: *const cb_id);
}
extern "C" {
    pub fn cn_queue_release_callback(: *mut cn_callback_entry);
}
extern "C" {
    pub fn cn_queue_free_dev(dev: *mut cn_queue_dev);
}
extern "C" {
    pub fn cn_cb_equal(: *const cb_id, : *const cb_id) -> c_int;
}
