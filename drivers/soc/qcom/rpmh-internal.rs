//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/qcom/rpmh-internal.h
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
// Copyright (c) 2016-2018, The Linux Foundation. All rights reserved.
//

pub const TCS_TYPE_NR: c_int = 4;
pub const MAX_CMDS_PER_TCS: c_int = 16;
pub const MAX_TCS_PER_TYPE: c_int = 3;

//
// struct tcs_group: group of Trigger Command Sets (TCS) to send state requests
// to the controller
//
// @drv:       The controller.
// @type:      Type of the TCS in this group - active, sleep, wake.
// @mask:      Mask of the TCSes relative to all the TCSes in the RSC.
// @offset:    Start of the TCS group relative to the TCSes in the RSC.
// @num_tcs:   Number of TCSes in this type.
// @ncpt:      Number of commands in each TCS.
// @req:       Requests that are sent from the TCS; only used for ACTIVE_ONLY
// transfers (could be on a wake/sleep TCS if we are borrowing for
// an ACTIVE_ONLY transfer).
// Start: grab drv->lock, set req, set tcs_in_use, drop drv->lock,
// trigger
// End: get irq, access req,
// grab drv->lock, clear tcs_in_use, drop drv->lock
// @slots:     Indicates which of @cmd_addr are occupied; only used for
// SLEEP / WAKE TCSs.  Things are tightly packed in the
// case that (ncpt < MAX_CMDS_PER_TCS).  That is if ncpt = 2 and
// MAX_CMDS_PER_TCS = 16 then bit[2] = the first bit in 2nd TCS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcs_group {
    pub drv: *mut rsc_drv,
    pub type: c_int,
    pub mask: u32,
    pub offset: u32,
    pub num_tcs: c_int,
    pub ncpt: c_int,
    pub req: [*const tcs_request; MAX_TCS_PER_TYPE],
    pub MAX_TCS_SLOTS): DECLARE_BITMAP(slots,,
}

//
// struct rpmh_request: the message to be sent to rpmh-rsc
//
// @msg: the request
// @cmd: the payload that will be part of the @msg
// @completion: triggered when request is done
// @dev: the device making the request
// @needs_free: check to free dynamically allocated request object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmh_request {
    pub msg: tcs_request,
    pub cmd: [tcs_cmd; MAX_RPMH_PAYLOAD],
    pub completion: *mut completion,
    pub dev: *const device,
    pub needs_free: bool,
}

//
// struct rpmh_ctrlr: our representation of the controller
//
// @cache: the list of cached requests
// @cache_lock: synchronize access to the cache data
// @dirty: was the cache updated since flush
// @batch_cache: Cache sleep and wake requests sent as batch
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmh_ctrlr {
    pub cache: list_head,
    pub cache_lock: spinlock_t,
    pub dirty: bool,
    pub batch_cache: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsc_ver {
    pub major: u32,
    pub minor: u32,
}

//
// struct rsc_drv: the Direct Resource Voter (DRV) of the
// Resource State Coordinator controller (RSC)
//
// @name:               Controller identifier.
// @base:               Start address of the DRV registers in this controller.
// @tcs_base:           Start address of the TCS registers in this controller.
// @id:                 Instance id in the controller (Direct Resource Voter).
// @num_tcs:            Number of TCSes in this DRV.
// @rsc_pm:             CPU PM notifier for controller.
// Used when solver mode is not present.
// @cpus_in_pm:         Number of CPUs not in idle power collapse.
// Used when solver mode and "power-domains" is not present.
// @genpd_nb:           PM Domain notifier for cluster genpd notifications.
// @tcs:                TCS groups.
// @tcs_in_use:         S/W state of the TCS; only set for ACTIVE_ONLY
// transfers, but might show a sleep/wake TCS in use if
// it was borrowed for an active_only transfer.  You
// must hold the lock in this struct (AKA drv->lock) in
// order to update this.
// @lock:               Synchronize state of the controller.  If RPMH's cache
// lock will also be held, the order is: drv->lock then
// cache_lock.
// @tcs_wait:           Wait queue used to wait for @tcs_in_use to free up a
// slot
// @client:             Handle to the DRV's client.
// @dev:                RSC device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsc_drv {
    pub name: *const c_char,
    pub base: *mut void __iomem,
    pub tcs_base: *mut void __iomem,
    pub id: c_int,
    pub num_tcs: c_int,
    pub rsc_pm: notifier_block,
    pub genpd_nb: notifier_block,
    pub cpus_in_pm: core::sync::atomic::AtomicI32,
    pub tcs: [tcs_group; TCS_TYPE_NR],
    pub MAX_TCS_NR): DECLARE_BITMAP(tcs_in_use,,
    pub lock: spinlock_t,
    pub tcs_wait: wait_queue_head_t,
    pub client: rpmh_ctrlr,
    pub dev: *mut device,
    pub ver: rsc_ver,
    pub regs: *mut u32,
}

extern "C" {
    pub fn rpmh_rsc_send_data(drv: *mut rsc_drv, msg: *const tcs_request) -> c_int;
}
extern "C" {
    pub fn rpmh_rsc_invalidate(drv: *mut rsc_drv);
}
extern "C" {
    pub fn rpmh_rsc_write_next_wakeup(drv: *mut rsc_drv);
}
extern "C" {
    pub fn rpmh_tx_done(msg: *const tcs_request);
}
extern "C" {
    pub fn rpmh_flush(ctrlr: *mut rpmh_ctrlr) -> c_int;
}
